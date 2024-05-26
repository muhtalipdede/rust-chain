# Basit Blockchain Uygulaması

Bu proje, Rust dilinde basit bir blockchain uygulamasıdır.

## Projenin Yapısı

Proje, bir blockchain'in temel bileşenlerini içerir:

- `Transaction`: İşlemleri temsil eder.
- `Block`: Blockchain'deki blokları temsil eder. Her blok, bir dizi işlem, bir zaman damgası, bir önceki bloğun hash'i, kendi hash'i ve bir nonce içerir.
- `Blockchain`: Bir dizi blok içerir ve yeni işlemler oluşturabilir, yeni bloklar ekleyebilir ve blockchain'in geçerliliğini kontrol edebilir.

## Kurulum

Bu projeyi çalıştırmak için, öncelikle Rust'ın kurulu olduğundan emin olun. Rust'ı [resmi web sitesinden](https://www.rust-lang.org/tools/install) indirebilirsiniz.

Proje dosyalarını indirdikten sonra, terminalden aşağıdaki komutu çalıştırın:

```bash
cargo run
```

## Kullanım

Proje çalıştırıldığında, aşağıdaki çıktıyı göreceksiniz:

```bash
Blockchain: Blockchain {
    chain: [
        Block {
            index: 0,
            timestamp: 1716749973194,
            transactions: [],
            previous_hash: "0",
            hash: "000098261b2fa10abe2109d81214edd25dca47ff2a7d6407ed3f0ef9b1a5b2be",
            nonce: 134873,
        },
        Block {
            index: 1,
            timestamp: 1716749974028,
            transactions: [
                Transaction {
                    sender_key: "Alice",
                    receiver_key: "Bob",
                    amount: 100,
                },
            ],
            previous_hash: "000098261b2fa10abe2109d81214edd25dca47ff2a7d6407ed3f0ef9b1a5b2be",
            hash: "0000ebea7d7fa1a93d095ba3b0ba2a61e12fe7f3c3743281455631666d76b455",
            nonce: 6782,
        },
        Block {
            index: 2,
            timestamp: 1716749974106,
            transactions: [
                Transaction {
                    sender_key: "Bob",
                    receiver_key: "Alice",
                    amount: 50,
                },
            ],
            previous_hash: "0000ebea7d7fa1a93d095ba3b0ba2a61e12fe7f3c3743281455631666d76b455",
            hash: "0000f5d33bf6493a25e02cc59da0bf4e58b3a4e55117fb1c8db95fb55d9a9cee",
            nonce: 15546,
        },
    ],
    pending_transaction: [],
    amount: 150,
}
```

## Katkıda Bulunma

Bu proje, her türlü katkıya açıktır. Eğer projeye katkıda bulunmak isterseniz, lütfen bir pull request oluşturun.

## Lisans

Bu proje MIT lisansı altında lisanslanmıştır. Daha fazla bilgi için [LICENSE](LICENSE) dosyasına bakabilirsiniz.
