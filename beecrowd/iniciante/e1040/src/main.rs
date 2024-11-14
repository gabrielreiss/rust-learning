use std::io;
fn main() {
    //normal
    let notas: Notas = coletar_notas();
    let r_media: f64 = media(notas);

    println!("Media: {:.1}", r_media);

    if r_media >= 7.0 {
        println!("Aluno aprovado.");
    } else if r_media < 7.0 && r_media >= 5.0 {
        println!("Aluno em exame.");
        let recuperacao:f64 = coletar_exame();
        println!("Nota do exame: {:.1}", recuperacao);

        let m_recup:f64 = media_recup(r_media, recuperacao);
        if m_recup >= 5.0 {
            println!("Aluno aprovado.");
            println!("Media final: {:.1}", m_recup);
        } else {
            println!("Aluno reprovado.");
            println!("Media final: {:.1}", m_recup);
        }
    } else {
        println!("Aluno reprovado.");
    }

    //exame
}

fn media(notas: Notas) -> f64 {
    return (notas.n1 * 2.0 + notas.n2 * 3.0 + notas.n3 * 4.0 + notas.n4 * 1.0) / 10.0;
}

fn media_recup(media:f64, recup:f64) -> f64 {
    return (media + recup) / 2.0;
}

fn coletar() -> String {
    let mut str_a:String = String::new();
    io::stdin().read_line(&mut str_a).expect("msg");
    return str_a;
}

fn coletar_notas() -> Notas {
    let str_a: String = coletar();
    let str_a:Vec<&str> = str_a.split(" ").collect();
    let data: Notas = Notas {
        n1: str_a[0].trim().parse::<f64>().expect("msg"),
        n2: str_a[1].trim().parse::<f64>().expect("msg"),
        n3: str_a[2].trim().parse::<f64>().expect("msg"),
        n4: str_a[3].trim().parse::<f64>().expect("msg")
    };
    return data;
}

fn coletar_exame() -> f64{
    let str_a: String = coletar();
    let str_a:f64 = str_a.trim().parse::<f64>().expect("msg");
    return str_a;
}

struct Notas {
    n1: f64,
    n2: f64,
    n3: f64,
    n4: f64
}