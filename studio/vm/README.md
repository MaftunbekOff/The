# twelfth_vm

Twelfth Hybrid Engine uchun **visual script virtual machine** — Bevy-dan mustaqil.

## Oqim

```
Editor GraphResource  →  GraphSnapshot  →  compile()  →  CompiledScript
                                                      →  VmRuntime::tick()
```

- **Faqat ulangan zanjir** ishlaydi (`Start` dan `out → in` simlar bo‘ylab).
- Editor terminali `VmEvent` larni ko‘rsatadi (`Execute`, `Wire`, `Finished`).

## Crate

```sh
cd The
cargo test -p twelfth_vm
```

## Keyingi qadamlar

- Tugun parametrlari (Log matni, Wait vaqti)
- O‘yin dunyosiga signal/event chiqarish
- `.twelfth` skript faylini saqlash / yuklash
