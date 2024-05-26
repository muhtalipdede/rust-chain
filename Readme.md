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
Blockchain {
    chain: [
        Block {
            index: 0,
            timestamp: 1716749557222,
            transactions: [],
            previous_hash: "0",
            hash: "0000f2947e177b30f5cf9cf7fdeb1307bd1bccc726f6651eeb6c06cd66ec3218",
            nonce: 7594,
        },
    ],
    transaction: [
        Transaction {
            sender_key: "Alice",
            receiver_key: "Bob",
            amount: 100,
        },
        Transaction {
            sender_key: "Bob",
            receiver_key: "Alice",
            amount: 50,
        },
    ],
    amount: 150,
}
```

## Katkıda Bulunma

Bu proje, her türlü katkıya açıktır. Eğer projeye katkıda bulunmak isterseniz, lütfen bir pull request oluşturun.

## Lisans

Bu proje MIT lisansı altında lisanslanmıştır. Daha fazla bilgi için [LICENSE](LICENSE) dosyasına bakabilirsiniz.
