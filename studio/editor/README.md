# Twelfth Editor

```sh
cargo run -p twelfth_editor --bin dev_editor
```

## Tugunlar

| Harakat | Qanday |
|---------|--------|
| **Qo‘shish** | Chap panel: `+ Start`, `+ Log`, `+ Wait`, `+ End` |
| **Sudrash** | Tugun ustiga bosib torting |
| **Tanlash** | Tugunni bosing (ko‘k ramka) |
| **Bog‘lash** | `out` portda **chap tugma** bosib ushlab, `in` portga olib **qo‘yish** |
| **Uzish** | Ulangan portda (`out` yoki `in`) **o‘ng tugma** |
| **Uzish** | Tugun tanlang → «Chiqishni uzish» |
| **O‘chirish** | Tugun tanlang → «O‘chirish» |
| **Fon** | Bo‘sh canvas — tanlovni bekor qilish |

**Run** — faqat `Start` dan **sim** (`out` → `in`) orqali ulangan zanjir ishlaydi.  
Masalan `Start → Wait → End`: `Wait` simda bo‘lmasa, u bajarilmaydi. Ulanmagan tugunlar o‘tkazib yuboriladi.
