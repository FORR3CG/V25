mod hestur;
mod stada;
mod eigandi;

use hestur::Hestur;

fn main() {
    let mut sleipnir = Hestur::new(100, "Gráni", 15, "laus", None);
    println!("{}", sleipnir);
    sleipnir.skra_eiganda("Geir", "ges@tskoli.is");
    sleipnir.breyta_stodu("leigður");
    println!("{}", sleipnir);
    sleipnir.breyta_netfangi_eiganda("ges@uskoli.is");
    sleipnir.breyta_stodu("ekki til leigu");
    println!("{}", sleipnir);
    sleipnir.breyta_stodu("veit ekki");
    println!("{}", sleipnir);
    
}
