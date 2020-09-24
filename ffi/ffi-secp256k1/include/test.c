#include <stdio.h>

#include "./secp256k1.h"

// 返回16进制字符代表的整数值
int hex2int(unsigned char x){
    if(x >= '0' && x <= '9'){
        return (x - '0');
    }
    if(x >= 'A' && x <= 'F'){
        return (x - 'A' + 10);
    }
    if(x >= 'a' && x <= 'f'){
        return (x - 'a' + 10);
    }
    return -1;
}
/** 测试主函数 */
int main(int argc, char** argv) {
    unsigned char prikeyhex[] = "9a9a6539856be209b8ea2adbd155c0919646d108515b60b7b13d6a79f1ae5174";
    int len = sizeof(prikeyhex) / 2;    // 私钥长度 - 32字节
    unsigned char prikey[len];          // 私钥存储
    int ii;                             // 索引值
    int ret;                            // 返回值

    unsigned char CPubKey[65];          // 公钥存储
    size_t clen;                        // 返回公钥长度

    secp256k1_context *secp256k1_context_sign;
    secp256k1_pubkey pubkey;            // secp256k1返回公钥
    // 将私钥字符串转换为字节存储
    for(ii = 0; ii < sizeof(prikeyhex); ii+=2){     
        prikey[ii/2] = hex2int(prikeyhex[ii]) * 16 + hex2int(prikeyhex[ii + 1]);
    }
    // 打印私钥
    printf("Private key: "); 
    for(ii = 0; ii < len; ii++)
    {
        printf("%02x",prikey[ii]);
    } 
    printf("\n");
    // 生成公钥
    secp256k1_context_sign = secp256k1_context_create(SECP256K1_CONTEXT_SIGN);
    ret = secp256k1_ec_pubkey_create(secp256k1_context_sign, &pubkey, prikey);  

    // 打印公钥
    if(ret){
        printf("Public key : ");  
        printf("[X(");
        for(ii = 63; ii >= 32; ii--){
            printf("%02x", pubkey.data[ii]);
        }
        printf("):Y(");
        for(ii = 31; ii >= 0; ii--){
            printf("%02x", pubkey.data[ii]);
        }
        printf(")]\n");
        // 获取压缩公钥
        clen = 65;
        secp256k1_ec_pubkey_serialize(secp256k1_context_sign, CPubKey, &clen, &pubkey, SECP256K1_EC_COMPRESSED);
        printf("Compressed key  : ");
        for(ii = 0; ii < clen; ii++){
            printf("%02x", CPubKey[ii]);
        }
        printf("\n");
        // 获取非压缩公钥
        clen = 65;
        secp256k1_ec_pubkey_serialize(secp256k1_context_sign, CPubKey, &clen, &pubkey, SECP256K1_EC_UNCOMPRESSED);
        printf("Uncompressed key: ");
        for(ii = 0; ii < clen; ii++){
            printf("%02x", CPubKey[ii]);
        }
        printf("\n");
    }
    if (secp256k1_context_sign) {
        secp256k1_context_destroy(secp256k1_context_sign);
    }
    return 0;
}