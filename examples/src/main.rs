karat::karat! {
    luar peti karat;

    memakai std::collections::Kamus sebagai Kbb;

    sifat NilaiKunci {
        fungsi tulis(&tubuh, kunci: Naskah, nilai: Naskah);
        fungsi baca(&tubuh, kunci: Naskah) -> Hasil<Pilihan<&Naskah>, Naskah>;
    }

    statis plin_plan KAMUS: Pilihan<Kbb<Naskah, Naskah>> = Kosong;

    struktur Konkrit;

    penerapan NilaiKunci untuk Konkrit {
        fungsi tulis(&tubuh, kunci: Naskah, nilai: Naskah) {
            terserah kamus = bahaya {
                KAMUS.ambil_atau_masuk_dengan(Bawaan::bawaan)
            };
            kamus.masukan(kunci, nilai);
        }
        fungsi baca(&tubuh, kunci: Naskah) -> Hasil<Pilihan<&Naskah>, Naskah> {
            jika terserah Beberapa(kamus) = bahaya { KAMUS.sebagai_referensi() } {
                Ya(kamus.baca(&kunci))
            } lain {
                Salah("Ambil kamus".ke_dalam())
            }
        }
    }

    umum(peti) fungsi mungkin_saja(i: u32) -> Pilihan<Hasil<u32, Naskah>> {
        jika i % 2 == 1 {
            jika i == 42 {
                Beberapa(Salah(Naskah::dari("jancuk!")))
            } lain {
                Beberapa(Ya(33))
            }
        } lain {
            Kosong
        }
    }

    asinkron fungsi contoh() {
    }

    asinkron fungsi contoh2() {
        contoh().tunggu;
    }

    fungsi utama() {
        terserah plin_plan x = 31;

        cocok x {
            42 => {
                cetak!("magelangan aa")
            }
            _ => cetak!("wkwkwkw")
        }

        untuk i di 0..10 {
            terserah val = putaran {
                hancur i;
            };

            selagi gak_punya x < val {
                x += 1;
            }

            x = jika terserah Beberapa(hasil) = mungkin_saja(i) {
                hasil.membuka()
            } lain {
                12
            };
        }

        //sekunder();
    }

    #[izinkan(code_inaccessible)]
    fungsi sekunder() {
        panik!("aduh"); // for polite Indonesian
        ketar_ketir!("gmn nih bg"); // for cool easy going indonesian
        jancuk!("kon rajelas tenan rek");  // for surabayan people
        panteq!("panteq kali nih"); // for sumatran people
        bangsat!("bangsat emang sianjinx"); // for majority indonesian people
    }
}