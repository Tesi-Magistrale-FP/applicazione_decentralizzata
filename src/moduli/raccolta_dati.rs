use rand::Rng;
use anyhow::Result;
use std::str::FromStr;
use chrono::{Datelike, Timelike, Utc};
use iota_streams::{
    app::transport::tangle::{client::Client, TangleAddress},
    app_channels::api::tangle::{Address, Author, Bytes},
};
use crate::moduli::streams;

// Raccolta dei dati e pubblicazione sul canale dati e log
pub async fn raccolta(id_app: i32, id_op: i32, did_produttore: String, did_consumatore: String, autore_dati: &mut Author<Client>, autore_log: &mut Author<Client>) -> Result<()>
{
    println!("\n---------------------------------------");
    println!("\nOPERAZIONE SUI DATI: RACCOLTA\n");

    // Controllo autorizzazione per eseguire la raccolta
    let esito_controllo: bool = streams::controlla_autorizzazione(id_app, id_op, did_produttore.clone(), did_consumatore.clone());
    
    if esito_controllo == true                                                                                      // Autorizzazione concessa
    {
        println!("- Autorizzazione concessa per eseguire l'operazione di raccolta!");

        let indirizzi: Vec<String> = streams::ind_ult_msg(id_app, id_op, did_produttore.clone());                   // Lettura degli indirizzi degli ultimi messaggi pubblicati sul canale dati e log
        let mut link_msg_prec_dati: TangleAddress = Address::from_str(&indirizzi[0])?;
        let mut link_msg_prec_log: TangleAddress = Address::from_str(&indirizzi[1])?;
        let mut contatore: i32 = 0;                                                                                 // Contatore che indica il numero di transazioni processate
        let coordinate: Vec<String> = genera_coordinate(10);                                                        // Genera casualmente i valori da scrivere sul canale dati
        let mut iteratore = coordinate.iter();
        while let Some(valore) = iteratore.next()                                                      				// Per ogni messaggio da scrivere sul canale
        {
            let (msg_link_dati, _seq_link) = autore_dati.send_signed_packet(          								// Creo il messaggio dati cifrato e firmato dall'Author e lo attacco al tangle
                &link_msg_prec_dati,
                &Bytes::default(),
                &Bytes(valore.as_bytes().to_vec()),
            ).await?;
            link_msg_prec_dati = msg_link_dati.clone();                                                             // Aggiorno il link al messaggio precedente per il canale dati

            let now = Utc::now();
            let timestamp: String = format!("{}/{}/{} - {}:{}:{}", now.year(), now.month(), now.minute(), now.hour(), now.minute(), now.second());
            let log: String = format!("{} | ID applicazione: {} | ID operazione: {} | DID produttore: {} | Link messaggio: {} | Raccolta dati", timestamp, id_app, id_op, did_produttore.clone(), msg_link_dati.clone());

            let (msg_link_log, _seq_link) = autore_log.send_signed_packet(          								// Creo il messaggio log cifrato e firmato dall'Author e lo attacco al tangle
                &link_msg_prec_log,
                &Bytes::default(),
                &Bytes(log.as_bytes().to_vec()),
            ).await?;
            link_msg_prec_log = msg_link_log.clone();																// Aggiorno il link al messaggio precedente per il canale log 

            contatore += 1;                                                                                 		// Incremento il contatore dei messaggi pubblicati
            
            println!("\n- {} messaggi scritti nel canale\n\tDati: {}\n\tLog: {}", contatore, valore, log);
        }
        println!("\n- Fine scrittura messaggi nel canale");

        // Aggiornamento indirizzi degli ultimi messaggi pubblicati sul canale dati e log
        streams::modifica_ind_ult_msg(id_app, id_op, did_produttore.clone(), link_msg_prec_dati.to_string().clone(), link_msg_prec_log.to_string().clone());
    }
    else                                                                                                            // Autorizzazione non concessa
    {
        println!("- Non è possibile eseguire l'operazione di raccolta per mancanza di autorizzazione!");
    }

    Ok(())
}

// Genera casualmente "max_esc" coordinate (latitudine, longitudine) e restituirle in un vettore di stringhe
fn genera_coordinate(max_esc: i32) -> Vec<String>                                             
{
    let mut coordinate = vec![];                                            										// Vettore che conterrà le coordinate (latitudine, longitudine) generate casualmente 
    for _i in 0..max_esc                                                            								// Genero casualmente i valori delle coordinate dell'utente (latitudine, longitudine) per ogni transazione
    {
        let lat = rand::thread_rng().gen_range(-90.0..=90.0);                       								// Generazione casuale dalla latitudine
        let lon = rand::thread_rng().gen_range(-180.0..=180.0);                     								// Generazione casuale della longitudine

        coordinate.push(format!("({}, {})", lat, lon));                                  							// Inserimento delle coordinate generate nel vettore
    }

    return coordinate;                                                                   							// Restituisco il vettore con tutte le coordinate generate casualmente
}