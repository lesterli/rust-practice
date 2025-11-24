"""
Streamlit UI for ArchLens
A simple web interface to browse and filter AI-classified engineering blog posts
"""

import streamlit as st
import json
from database import get_posts, get_all_sources, init_database, load_sources
from datetime import datetime

# Initialize database on startup
init_database()
load_sources()

# Page config
st.set_page_config(
    page_title="ArchLens",
    page_icon="🔍",
    layout="wide"
)

# Custom CSS
st.markdown("""
<style>
.main {
    padding: 2rem;
}

.post-card {
    padding: 1.5rem;
    margin: 1rem 0;
    border-radius: 0.5rem;
    border: 1px solid #e0e0e0;
    background-color: white;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.post-title {
    font-size: 1.25rem;
    font-weight: 600;
    margin-bottom: 0.5rem;
    color: #1a1a1a;
}

.post-meta {
    font-size: 0.875rem;
    color: #666;
    margin-bottom: 1rem;
}

.post-content {
    font-size: 0.95rem;
    line-height: 1.6;
    margin-bottom: 1rem;
    color: #333;
}

.category-badge {
    display: inline-block;
    padding: 0.25rem 0.75rem;
    border-radius: 1rem;
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    margin-right: 0.5rem;
}

.category-WHAT {
    background-color: #e3f2fd;
    color: #1976d2;
}

.category-HOW {
    background-color: #f3e5f5;
    color: #7b1fa2;
}

.category-WHY {
    background-color: #fff3e0;
    color: #e65100;
}

.tag {
    display: inline-block;
    padding: 0.25rem 0.5rem;
    margin: 0.25rem;
    background-color: #f5f5f5;
    border-radius: 0.25rem;
    font-size: 0.75rem;
    color: #666;
}

.stSelectbox, .stTextInput {
    margin-bottom: 1rem;
}
</style>
""", unsafe_allow_html=True)

def display_post(post):
    """Render a single post card"""
    # Format published date
    published = post['published_at']
    if published:
        try:
            dt = datetime.fromisoformat(published.replace('Z', '+00:00'))
            published_str = dt.strftime('%b %d, %Y')
        except:
            published_str = "Unknown date"
    else:
        published_str = "Unknown date"

    # Render card
    st.markdown(f"""
    <div class="post-card">
        <div class="post-title">{post['title']}</div>
        <div class="post-meta">
            <strong>{post.get('source_id', 'Unknown')}</strong> • {published_str}
            <span class="category-badge category-{post['ai_category']}">
                {post['ai_category']}
            </span>
            <span style="font-size: 0.75rem; color: #999; margin-left: 0.5rem;">
                {post['ai_confidence']} confidence
            </span>
        </div>
        <div class="post-content">
            {post['content_summary'][:300]}...
        </div>
    </div>
    """, unsafe_allow_html=True)

    # Tags
    tags = post.get('ai_tags', [])
    if tags:
        tags_html = " ".join([f"<span class='tag'>{tag}</span>" for tag in tags])
        st.markdown(tags_html, unsafe_allow_html=True)

    # Read link button
    st.link_button("🔗 Read Original", post['url'], use_container_width=True)

    st.markdown("---")

def main():
    """Main Streamlit app"""
    st.title("🔍 ArchLens")
    st.markdown("*See engineering blogs the way architects do*")
    st.markdown("")

    # Sidebar filters
    st.sidebar.header("Filters")

    # Get data
    sources = get_all_sources()
    source_options = {f"{name} ({sid})": sid for sid, name, url in sources}
    source_options["All Sources"] = None

    selected_source = st.sidebar.selectbox(
        "Source",
        options=list(source_options.keys()),
        index=0
    )

    # Category filter
    categories = ["All", "WHAT", "HOW", "WHY"]
    selected_category = st.sidebar.selectbox(
        "Category",
        categories,
        index=0
    )

    # Search
    keywords = st.sidebar.text_input("Search Keywords")

    # Build filters
    filters = {}
    if selected_source and selected_source != "All Sources":
        filters['source_id'] = source_options[selected_source]
    if selected_category and selected_category != "All":
        filters['category'] = selected_category
    if keywords:
        filters['keywords'] = keywords

    # Fetch posts
    posts = get_posts(filters)

    # Display results
    st.subheader(f"📚 {len(posts)} Posts Found")

    if not posts:
        st.info("No posts found. Try running the ingestion pipeline: `uv run ingest.py`")
    else:
        for post in posts:
            display_post(post)

    # Footer
    st.sidebar.markdown("---")
    st.sidebar.markdown("*Run ingestion:* `uv run ingest.py`")

if __name__ == "__main__":
    main()
