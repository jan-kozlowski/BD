use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    io::{Read, Write},
};

#[derive(PartialEq)]
struct Autorstwo {
    praca: u16,
    autor: String,
}

#[derive(Debug)]
struct Autor {
    id: String,
    ryzyko: u8,
    sloty: String,
}

#[derive(PartialEq)]
struct Praca {
    id: u16,
    tytul: String,
    rok: u16,
    autorzy: u8,
    punkty: u8,
}

const AUTORSTWA_ROZMIAR: usize = 630;
const AUTORZY_ROZMIAR: usize = 125;
const PRACE_ROZMIAR: usize = 389;

fn wczytaj_autorstwo() -> Result<Vec<Autorstwo>, String> {
    let mut autorstwa = Vec::new();
    let mut input_file =
        std::fs::File::open("./assets/Autorstwo.csv").map_err(|e| e.to_string())?;

    let mut rows_parsed = 0;
    let mut lines = String::new();
    input_file
        .read_to_string(&mut lines)
        .map_err(|e| e.to_string())?;

    for line in lines.lines() {
        // let (id, tytul, rok, autorzy, punkty ) = get_line_data(line).ok_or(format!("Błąd konwertowania wiersza {}", rows_selected + 1))?;
        let autorstwo =
            get_autorstwo(line).ok_or(format!("Błąd konwertowania wiersza {}", rows_parsed + 1))?;
        autorstwa.push(autorstwo);
        rows_parsed += 1;
    }

    assert_eq!(rows_parsed, AUTORSTWA_ROZMIAR);
    assert_eq!(autorstwa.len(), AUTORSTWA_ROZMIAR);
    Ok(autorstwa)
}

fn wczytaj_autorow() -> Result<Vec<Autor>, String> {
    let mut autorzy = Vec::new();
    let mut input_file = std::fs::File::open("./assets/Autorzy.csv").map_err(|e| e.to_string())?;

    let mut rows_parsed = 0;
    let mut lines = String::new();
    input_file
        .read_to_string(&mut lines)
        .map_err(|e| e.to_string())?;

    for line in lines.lines() {
        // let (id, tytul, rok, autorzy, punkty ) = get_line_data(line).ok_or(format!("Błąd konwertowania wiersza {}", rows_selected + 1))?;
        let autor =
            get_autor(line).ok_or(format!("Błąd konwertowania wiersza {}", rows_parsed + 1))?;
        autorzy.push(autor);
        rows_parsed += 1;
    }

    assert_eq!(rows_parsed, AUTORZY_ROZMIAR);
    assert_eq!(autorzy.len(), AUTORZY_ROZMIAR);
    Ok(autorzy)
}

fn wczytaj_prace() -> Result<HashMap<u16, Praca>, String> {
    let mut prace = HashMap::new();
    let mut input_file = std::fs::File::open("./assets/Prace.csv").map_err(|e| e.to_string())?;

    let mut rows_parsed = 0;
    let mut lines = String::new();
    input_file
        .read_to_string(&mut lines)
        .map_err(|e| e.to_string())?;

    for line in lines.lines() {
        let praca =
            get_praca(line).ok_or(format!("Błąd konwertowania wiersza {}", rows_parsed + 1))?;
        prace.insert(praca.id, praca);
        rows_parsed += 1;
    }

    assert_eq!(rows_parsed, PRACE_ROZMIAR);
    assert_eq!(prace.len(), PRACE_ROZMIAR);
    assert_eq!(prace.values().len(), PRACE_ROZMIAR);
    Ok(prace)
}

fn make_query_file_2(
    autorstwa: &Vec<Autorstwo>,
    autorzy: &Vec<Autor>,
    prace: &HashMap<u16, Praca>,
) -> Result<(), std::io::Error> {
    let mut output_file = std::fs::File::create("./assets/Publikacje_2.sql")?;

    writeln!(
        &mut output_file,
        "CREATE TABLE autorstwa (
    praca NUMBER(3, 0) NOT NULL,
    autor VARCHAR2(29) NOT NULL,
    PRIMARY KEY (praca, autor)
);

CREATE TABLE autorzy (
    autor VARCHAR2(29) PRIMARY KEY,
    ryzyko NUMBER(1, 0) NOT NULL,
    sloty VARCHAR2(4) NOT NULL
);

CREATE TABLE prace (
    id NUMBER(3, 0) PRIMARY KEY,
    tytul VARCHAR2(197) NOT NULL,
    rok NUMBER(4, 0) NOT NULL,
    autorzy NUMBER(2, 0) NOT NULL,
    punkty NUMBER(3, 0) NOT NULL
);"
    )?;

    writeln!(&mut output_file)?;

    let mut autorstwa_selected = 0;
    let mut autorzy_selected = 0;
    let mut prace_selected = 0;

    for autorstwo in autorstwa {
        writeln!(
            &mut output_file,
            "INSERT INTO autorstwa (praca, autor) VALUES ({}, '{}');",
            autorstwo.praca, autorstwo.autor
        )?;
        autorstwa_selected += 1;
    }

    writeln!(&mut output_file)?;

    for autor in autorzy {
        writeln!(
            &mut output_file,
            "INSERT INTO autorzy (autor, ryzyko, sloty) VALUES ('{}', {}, '{}');",
            autor.id, autor.ryzyko, autor.sloty
        )?;

        autorzy_selected += 1;
    }

    writeln!(&mut output_file)?;

    for praca in prace.values() {
        writeln!(
            &mut output_file,
            "INSERT INTO prace (id, tytul, rok, autorzy, punkty) VALUES ({}, '{}', {}, {}, {});",
            praca.id, praca.tytul, praca.rok, praca.autorzy, praca.punkty
        )?;

        prace_selected += 1;
    }

    writeln!(&mut output_file,
    "
select distinct A.autor, B.autor as wspolautor FROM autorstwa A, autorstwa B WHERE A.autor in (SELECT autorzy.autor from autorzy WHERE autor NOT IN (SELECT distinct autorstwa.autor FROM autorstwa JOIN prace ON autorstwa.praca = prace.id WHERE rok = 2020) AND ryzyko = 1) AND A.praca = B.praca AND A.autor != B.autor order by A.autor;
")?;

    assert_eq!(autorstwa_selected, AUTORSTWA_ROZMIAR);
    assert_eq!(autorzy_selected, AUTORZY_ROZMIAR);
    assert_eq!(prace_selected, PRACE_ROZMIAR);

    Ok(())
}

fn publikacje_2() -> Result<(), String> {
    let autorstwa = wczytaj_autorstwo()?;
    let autorzy = wczytaj_autorow()?;
    let prace = wczytaj_prace()?;

    let mut publikacje = HashMap::new();

    for autor in &autorzy {
        if autor.ryzyko != 1 {
            continue;
        }

        let mut jest_wybrany = true;
        let mut publikacje_autora = Vec::new();

        for autorstwo in &autorstwa {
            if autorstwo.autor != autor.id {
                continue;
            }

            let praca = prace.get(&autorstwo.praca).ok_or("Brak pracy")?;
            publikacje_autora.push(praca);

            if praca.rok == 2020 {
                jest_wybrany = false;
            }
        }

        if jest_wybrany && publikacje_autora.len() > 0 {
            publikacje.insert(autor.id.clone(), publikacje_autora);
        }
    }

    let mut lines = 0;
    for autor in publikacje.keys() {
        println!("wybrany: {}", autor);
    }
    println!("liczba wybranych: {}", publikacje.len());

    for (autor, publikacje_autora) in publikacje {
        let mut wspolpracownicy = HashSet::new();

        for autorstwo in &autorstwa {
            for publikacja in &publikacje_autora {
                if autorstwo.praca == publikacja.id && autorstwo.autor != autor {
                    wspolpracownicy.insert(&autorstwo.autor);
                }
            }
        }

        for wspolpracownik in wspolpracownicy {
            lines += 1;
            println!("autor: {}, współpracownik: {}", autor, wspolpracownik);
        }
    }
    println!("wypisano: {lines}");

    make_query_file_2(&autorstwa, &autorzy, &prace).map_err(|e| e.to_string())
}

fn make_query_file_3(
    autorstwa: &Vec<Autorstwo>,
    autorzy: &Vec<Autor>,
    prace: &HashMap<u16, Praca>,
) -> Result<(), std::io::Error> {
    let mut output_file = std::fs::File::create("./assets/Publikacje_3.sql")?;

    writeln!(
        &mut output_file,
        "CREATE TABLE autorstwa (
    praca NUMBER(3, 0) NOT NULL,
    autor VARCHAR2(29) NOT NULL,
    PRIMARY KEY (praca, autor)
);

CREATE TABLE autorzy (
    autor VARCHAR2(29) PRIMARY KEY,
    ryzyko NUMBER(1, 0) NOT NULL,
    sloty VARCHAR2(4) NOT NULL
);

CREATE TABLE prace (
    id NUMBER(3, 0) PRIMARY KEY,
    tytul VARCHAR2(197) NOT NULL,
    rok NUMBER(4, 0) NOT NULL,
    autorzy NUMBER(2, 0) NOT NULL,
    punkty NUMBER(3, 0) NOT NULL
);"
    )?;

    writeln!(&mut output_file)?;

    let mut autorstwa_selected = 0;
    let mut autorzy_selected = 0;
    let mut prace_selected = 0;

    for autorstwo in autorstwa {
        writeln!(
            &mut output_file,
            "INSERT INTO autorstwa (praca, autor) VALUES ({}, '{}');",
            autorstwo.praca, autorstwo.autor
        )?;
        autorstwa_selected += 1;
    }

    writeln!(&mut output_file)?;

    for autor in autorzy {
        writeln!(
            &mut output_file,
            "INSERT INTO autorzy (autor, ryzyko, sloty) VALUES ('{}', {}, '{}');",
            autor.id, autor.ryzyko, autor.sloty
        )?;

        autorzy_selected += 1;
    }

    writeln!(&mut output_file)?;

    for praca in prace.values() {
        writeln!(
            &mut output_file,
            "INSERT INTO prace (id, tytul, rok, autorzy, punkty) VALUES ({}, '{}', {}, {}, {});",
            praca.id, praca.tytul, praca.rok, praca.autorzy, praca.punkty
        )?;

        prace_selected += 1;
    }

    writeln!(&mut output_file,
    "
SELECT autor, SUM(wartosc) as wynik FROM (SELECT A.autor, praca, W.id, W.wartosc, ROW_NUMBER() OVER (PARTITION BY A.autor ORDER BY W.wartosc desc) as rn FROM autorzy A JOIN autorstwa ON A.autor = autorstwa.autor JOIN (SELECT id, (punkty / autorzy) AS wartosc from prace) W ON W.id = autorstwa.praca) WHERE rn <= 4 GROUP BY autor;

DROP TABLE autorzy;
DROP TABLE autorstwa;
DROP TABLE prace;"
    )?;

    assert_eq!(autorstwa_selected, AUTORSTWA_ROZMIAR);
    assert_eq!(autorzy_selected, AUTORZY_ROZMIAR);
    assert_eq!(prace_selected, PRACE_ROZMIAR);

    Ok(())
}

fn publikacje_3() -> Result<(), String> {
    let autorstwa = wczytaj_autorstwo()?;
    let autorzy = wczytaj_autorow()?;
    let prace = wczytaj_prace()?;
    let mut inserted = 0;

    let mut wartosci = HashMap::new();
    for (id, praca) in &prace {
        wartosci.insert(id, praca.punkty as f32 / praca.autorzy as f32);
    }

    let mut prace_autora = HashMap::new();
    for autor in &autorzy {
        let mut publikacje = Vec::new();
        for autorstwo in &autorstwa {
            if autorstwo.autor == autor.id {
                let wartosc = wartosci.get(&autorstwo.praca).ok_or("Błąd")?.clone();
                publikacje.push((autorstwo.praca, wartosc));
                inserted += 1;
            }
        }

        publikacje.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let mut wybrane = Vec::new();
        let mut odrzucone = Vec::new();

        for publikacja in publikacje {
            if wybrane.len() < 4 {
                wybrane.push(publikacja);
            } else {
                odrzucone.push(publikacja);
            }
        }

        prace_autora.insert(autor.id.clone(), (wybrane, odrzucone));
    }

    let mut selected = 0;
    let mut discarded = 0;
    let mut printed = 0;
    for autor in &autorzy {
        let (wybrane, odrzucone) = prace_autora.get(&autor.id).ok_or("Błąd")?;
        let mut wynik_autora = 0.;
        assert!(wybrane.len() <= 4);
        selected += wybrane.len();
        discarded += odrzucone.len();
        for (id, wartosc) in wybrane {
            wynik_autora += wartosc;
            // println!("autor: {}, praca: {}, wartosc: {}",
            //     autor.id, id, wartosc);

            for odrzucona in odrzucone {
                assert!(wartosc.clone() >= odrzucona.1);
            }
        }

        // for (id, wartosc) in odrzucone {

        //     println!("autor: {}, praca: {}, wartosc: {}",
        //         autor.id, id, wartosc);
        // }

        if wybrane.len() > 0 {
            printed += 1;
            println!("AUTOR: {}, WYNIK: {}", autor.id, wynik_autora);
        }
    }

    assert_eq!(selected + discarded, 628);
    assert_eq!(inserted, 628);
    // assert_eq!(printed, 125);
    println!("wybrano: {selected}");
    println!("odrzucono: {discarded}");
    println!("total: {}", selected + discarded);
    println!("wypisano: {printed}");

    make_query_file_3(&autorstwa, &autorzy, &prace).map_err(|e| e.to_string())
}

fn make_query_file_4(
    autorstwa: &Vec<Autorstwo>,
    autorzy: &Vec<Autor>,
    prace: &HashMap<u16, Praca>,
) -> Result<(), std::io::Error> {
    let mut output_file = std::fs::File::create("./assets/Publikacje_4.sql")?;

    writeln!(
        &mut output_file,
        "CREATE TABLE autorstwa (
    praca NUMBER(3, 0) NOT NULL,
    autor VARCHAR2(29) NOT NULL,
    PRIMARY KEY (praca, autor)
);

CREATE TABLE autorzy (
    autor VARCHAR2(29) PRIMARY KEY,
    ryzyko NUMBER(1, 0) NOT NULL,
    sloty VARCHAR2(4) NOT NULL
);

CREATE TABLE prace (
    id NUMBER(3, 0) PRIMARY KEY,
    tytul VARCHAR2(197) NOT NULL,
    rok NUMBER(4, 0) NOT NULL,
    autorzy NUMBER(2, 0) NOT NULL,
    punkty NUMBER(3, 0) NOT NULL
);"
    )?;

    writeln!(&mut output_file)?;

    let mut autorstwa_selected = 0;
    let mut autorzy_selected = 0;
    let mut prace_selected = 0;

    for autorstwo in autorstwa {
        writeln!(
            &mut output_file,
            "INSERT INTO autorstwa (praca, autor) VALUES ({}, '{}');",
            autorstwo.praca, autorstwo.autor
        )?;
        autorstwa_selected += 1;
    }

    writeln!(&mut output_file)?;

    for autor in autorzy {
        writeln!(
            &mut output_file,
            "INSERT INTO autorzy (autor, ryzyko, sloty) VALUES ('{}', {}, '{}');",
            autor.id, autor.ryzyko, autor.sloty
        )?;

        autorzy_selected += 1;
    }

    writeln!(&mut output_file)?;

    for praca in prace.values() {
        writeln!(
            &mut output_file,
            "INSERT INTO prace (id, tytul, rok, autorzy, punkty) VALUES ({}, '{}', {}, {}, {});",
            praca.id, praca.tytul, praca.rok, praca.autorzy, praca.punkty
        )?;

        prace_selected += 1;
    }

    writeln!(&mut output_file,
    "
SELECT distinct autor, LEVEL-1 FROM autorstwa START WITH autor = 'Pilipczuk Mi' CONNECT BY NOCYCLE PRIOR praca = praca AND PRIOR autor != autor;

DROP TABLE autorzy;
DROP TABLE autorstwa;
DROP TABLE prace;"
    )?;

    assert_eq!(autorstwa_selected, AUTORSTWA_ROZMIAR);
    assert_eq!(autorzy_selected, AUTORZY_ROZMIAR);
    assert_eq!(prace_selected, PRACE_ROZMIAR);

    Ok(())
}

fn policz(
    liczby: &mut HashMap<String, i32>,
    imie_autora: String,
    obliczeni: &mut HashSet<String>,
    autorstwa: &Vec<Autorstwo>,
    wartosc: i32,
) {
    assert!(obliczeni.contains(&imie_autora));

    let mut prace_autora = Vec::new();
    let mut wspolautorzy = Vec::new();

    for autorstwo in autorstwa {
        if autorstwo.autor == imie_autora.clone() {
            prace_autora.push(autorstwo.praca);
        }
    }

    for praca_autora in prace_autora {
        for autorstwo in autorstwa {
            if autorstwo.praca != praca_autora || obliczeni.contains(&autorstwo.autor) {
                continue;
            }
            assert_ne!(autorstwo.autor, imie_autora);
            wspolautorzy.push(autorstwo.autor.clone());
            liczby.insert(autorstwo.autor.clone(), wartosc + 1);
            obliczeni.insert(autorstwo.autor.clone());
        }
    }

    for wspolautor in wspolautorzy {
        let val = liczby.get(&wspolautor).unwrap();
        // println!("autor: {}, wartosc: {val}", imie_autora);
        assert_eq!(val.clone(), wartosc + 1);
        policz(liczby, wspolautor, obliczeni, autorstwa, wartosc + 1);
    }
}

fn publikacje_4() -> Result<(), String> {
    let autorstwa = wczytaj_autorstwo()?;
    let autorzy = wczytaj_autorow()?;
    let prace = wczytaj_prace()?;

    let mut liczby = HashMap::new();
    liczby.insert(String::from("Pilipczuk Mi"), 0);

    policz(
        &mut liczby,
        String::from("Pilipczuk Mi"),
        &mut HashSet::from([String::from("Pilipczuk Mi")]),
        &autorstwa,
        0,
    );

    assert_eq!(liczby.get("Pilipczuk Mi").unwrap().clone(), 0);
    let mut prace_autora = Vec::new();
    for autorstwo in &autorstwa {
        if autorstwo.autor == "Pilipczuk Mi" {
            prace_autora.push(autorstwo.praca);
        }
    }
    assert_eq!(prace_autora.len(), 48);

    for praca_autora in &prace_autora {
        for autorstwo in &autorstwa {
            if autorstwo.praca != *praca_autora || autorstwo.autor == "Pilipczuk Mi" {
                continue;
            }
            assert_eq!(liczby.get(&autorstwo.autor).unwrap().clone(), 1);
        }
    }

    let mut wyniki = HashMap::new();

    for autor in &autorzy {
        let text = match liczby.get(&autor.id) {
            None => String::from("NULL"),
            Some(liczba) => String::from(liczba.to_string()),
        };

        match wyniki.get(&text) {
            None => {
                wyniki.insert(text.clone(), 1);
            }
            Some(wynik) => {
                wyniki.insert(text.clone(), wynik + 1);
            }
        }

        println!("{}: {}", autor.id, text);
    }

    println!("---------------------");
    let null_suma = wyniki.get("NULL").unwrap();
    println!("NULL: {null_suma}");
    let mut suma = 0;
    let mut liczba = 0;
    while let Some(wynik) = wyniki.get(&liczba.to_string()) {
        println!("{liczba}: {wynik}");
        suma += wynik;
        liczba += 1;
    }


    assert_eq!(null_suma + suma, 125);

    make_query_file_4(&autorstwa, &autorzy, &prace).map_err(|e| e.to_string())
}

fn main() -> Result<(), String> {
    publikacje_4()
}

fn get_autorstwo(line: &str) -> Option<Autorstwo> {
    let mut split = line.split(',');
    let praca: u16 = split.next()?.parse().ok()?;
    let autor: String = split.next()?.parse().ok()?;
    let koniec = split.next();

    match koniec {
        Some(_v) => None,
        None => Some(Autorstwo {
            praca: praca,
            autor: autor,
        }),
    }
}

fn get_autor(line: &str) -> Option<Autor> {
    let mut in_quotes = false;
    let mut fields = Vec::new();
    let mut current = String::new();

    for c in line.chars() {
        match c {
            '"' => in_quotes = !in_quotes,
            ',' if !in_quotes => {
                fields.push(current.trim_matches('"').to_string());
                current.clear();
            }
            _ => current.push(c),
        }
    }
    fields.push(current.trim_matches('"').to_string());

    if fields.len() != 3 {
        return None;
    }

    Some(Autor {
        id: fields[0].clone(),
        ryzyko: fields[1].parse().ok()?,
        sloty: fields[2].clone(),
    })
}

fn get_praca(line: &str) -> Option<Praca> {
    let mut split = line.split(';');
    let id: u16 = split.next()?.parse().ok()?;
    let tytul: String = split.next()?.parse().ok()?;
    let rok: u16 = split.next()?.parse().ok()?;
    let autorzy: u8 = split.next()?.parse().ok()?;
    let punkty: u8 = split.next()?.parse().ok()?;
    let koniec = split.next();

    match koniec {
        Some(_v) => None,
        None => Some(Praca {
            id: id,
            tytul: tytul,
            rok: rok,
            autorzy: autorzy,
            punkty: punkty,
        }),
    }
}
