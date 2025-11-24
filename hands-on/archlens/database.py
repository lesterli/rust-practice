"""
Database setup and operations for ArchLens
"""

import sqlite3
from pathlib import Path
import json

DB_PATH = Path("archlens.db")

def init_database():
    """Initialize database with required tables"""
    conn = sqlite3.connect(DB_PATH)
    cursor = conn.cursor()

    # Create sources table
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS sources (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            url TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
    """)

    # Create posts table
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS posts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            source_id TEXT,
            title TEXT NOT NULL,
            url TEXT UNIQUE NOT NULL,
            author TEXT,
            published_at TIMESTAMP,
            content_summary TEXT,
            ai_category TEXT,
            ai_confidence TEXT,
            ai_tags TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (source_id) REFERENCES sources(id)
        )
    """)

    # Create index for faster queries
    cursor.execute("""
        CREATE INDEX IF NOT EXISTS idx_posts_published
        ON posts(published_at DESC)
    """)

    cursor.execute("""
        CREATE INDEX IF NOT EXISTS idx_posts_category
        ON posts(ai_category)
    """)

    conn.commit()
    conn.close()
    print("✅ Database initialized")

def load_sources():
    """Load sources from JSON file into database"""
    with open('sources.json', 'r') as f:
        sources = json.load(f)

    conn = sqlite3.connect(DB_PATH)
    cursor = conn.cursor()

    # Upsert sources
    for source in sources:
        cursor.execute("""
            INSERT OR REPLACE INTO sources (id, name, url)
            VALUES (?, ?, ?)
        """, (source['id'], source['name'], source['url']))

    conn.commit()
    conn.close()
    print(f"✅ Loaded {len(sources)} sources into database")

def get_all_sources():
    """Get all sources from database"""
    conn = sqlite3.connect(DB_PATH)
    cursor = conn.cursor()
    cursor.execute("SELECT id, name, url FROM sources ORDER BY name")
    sources = cursor.fetchall()
    conn.close()
    return sources

def check_post_exists(url):
    """Check if post URL already exists in database"""
    conn = sqlite3.connect(DB_PATH)
    cursor = conn.cursor()
    cursor.execute("SELECT 1 FROM posts WHERE url = ?", (url,))
    exists = cursor.fetchone() is not None
    conn.close()
    return exists

def insert_post(post_data):
    """Insert a new post into database"""
    conn = sqlite3.connect(DB_PATH)
    cursor = conn.cursor()

    cursor.execute("""
        INSERT INTO posts (
            source_id, title, url, author, published_at,
            content_summary, ai_category, ai_confidence, ai_tags
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
    """, (
        post_data['source_id'],
        post_data['title'],
        post_data['url'],
        post_data.get('author'),
        post_data.get('published_at'),
        post_data.get('content_summary'),
        post_data['ai_category'],
        post_data['ai_confidence'],
        json.dumps(post_data.get('ai_tags', []))
    ))

    conn.commit()
    conn.close()

def get_posts(filters=None):
    """
    Get posts with optional filters
    filters: dict with keys: category, keywords, source_id
    """
    conn = sqlite3.connect(DB_PATH)
    cursor = conn.cursor()

    query = "SELECT * FROM posts WHERE 1=1"
    params = []

    if filters:
        if filters.get('category'):
            query += " AND ai_category = ?"
            params.append(filters['category'])

        if filters.get('source_id'):
            query += " AND source_id = ?"
            params.append(filters['source_id'])

        if filters.get('keywords'):
            query += " AND (title LIKE ? OR ai_tags LIKE ?)"
            keyword = f"%{filters['keywords']}%"
            params.extend([keyword, keyword])

    query += " ORDER BY published_at DESC LIMIT 100"

    cursor.execute(query, params)
    columns = [desc[0] for desc in cursor.description]
    posts = [dict(zip(columns, row)) for row in cursor.fetchall()]
    conn.close()

    # Parse JSON tags
    for post in posts:
        if post['ai_tags']:
            post['ai_tags'] = json.loads(post['ai_tags'])
        else:
            post['ai_tags'] = []

    return posts

if __name__ == "__main__":
    init_database()
    load_sources()
    print("\n📊 Database ready!")
