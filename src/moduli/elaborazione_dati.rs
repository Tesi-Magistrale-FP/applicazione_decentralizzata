use anyhow::Result;
use std::str::FromStr;
use chrono::{Datelike, Timelike, Utc};
use iota_streams::{
    app::transport::tangle::client::Client,
    app_channels::api::tangle::{Address, Subscriber, Bytes, MessageContent},
};
use crate::moduli::streams;

// Accesso ai dati pubblicati sul canale dati e log
pub async fn elaborazione(id_app: i32, id_op: i32, did_produttore: String, did_consumatore: String, iscritto_dati: &mut Subscriber<Client>, iscritto_log: &mut Subscriber<Client>) -> Result<()>
{
    // Controllo autorizzazione per eseguire l'accesso
    let esito_controllo: bool = streams::controlla_autorizzazione(id_app, id_op, did_produttore.clone(), did_consumatore.clone());
    
    if esito_controllo == true                                                                                      // Autorizzazione concessa
    {
        println!("\n---------------------------------------");
        println!("\nOPERAZIONE SUI DATI: ACCESSO\n");
        println!("- Autorizzazione concessa per eseguire l'operazione di accesso!");

        // Lettura ed elaborazione dei messaggi del canale dati
        let msg_letti_dati = iscritto_dati.fetch_all_next_msgs().await;        										// Lettura di tutti i messaggi pubblicati sul canale privato
        let msg_elab_dati = msg_letti_dati                    														// Iterazione su tutti i messaggi dati letti per ottenere il loro contenuto
        .iter()
        .map(|msg| {
            let content = &msg.body;
            match content {
                MessageContent::SignedPacket {
                    pk: _,
                    public_payload: _,
                    masked_payload,
                } => String::from_utf8(masked_payload.0.to_vec()).unwrap(),
                _ => String::default(),
            }
        })
        .filter(|s| s != &String::default())
        .collect::<Vec<String>>();

        println!("- Messaggi dati letti:");
        for messaggio in msg_elab_dati.iter()
        {
            println!("\t- Messaggio: {}", messaggio);
        }

        // Lettura ed elaborazione dei messaggi del canale log
        let msg_letti_log = iscritto_log.fetch_all_next_msgs().await;        										// Lettura di tutti i messaggi pubblicati sul canale privato
        let msg_elab_log = msg_letti_log                    														// Iterazione su tutti i messaggi dati letti per ottenere il loro contenuto
        .iter()
        .map(|msg| {
            let content = &msg.body;
            match content {
                MessageContent::SignedPacket {
                    pk: _,
                    public_payload: _,
                    masked_payload,
                } => String::from_utf8(masked_payload.0.to_vec()).unwrap(),
                _ => String::default(),
            }
        })
        .filter(|s| s != &String::default())
        .collect::<Vec<String>>();

        println!("- Messaggi log letti:");
        for messaggio in msg_elab_log.iter()
        {
            println!("\t- Messaggio: {}", messaggio);
        }

        // Scrittura sul canale di log dell'operazione di lettura eseguita
        let indirizzi: Vec<String> = streams::ind_ult_msg(id_app, id_op, did_produttore.clone());                   // Lettura degli indirizzi degli ultimi messaggi pubblicati sul canale dati e log
        let now = Utc::now();
        let timestamp: String = format!("{}/{}/{} - {}:{}:{}", now.year(), now.month(), now.minute(), now.hour(), now.minute(), now.second());
        let log: String = format!("{} | ID applicazione: {} | ID operazione: {} | DID produttore: {} | DID consumatore: {} | Accesso ai dati", timestamp, id_app, id_op, did_produttore, did_consumatore);

        let (msg_link_log, _seq_link) = iscritto_log.send_signed_packet(          									// Creo il messaggio log cifrato e firmato dal sSubscriber e lo attacco al tangle
            &Address::from_str(&indirizzi[1].clone()).unwrap(),
            &Bytes::default(),
            &Bytes(log.as_bytes().to_vec()),
        ).await?;

        // Aggiornamento indirizzi degli ultimi messaggi pubblicati sul canale dati e log
        streams::modifica_ind_ult_msg(id_app, id_op, did_produttore.clone(), indirizzi[0].clone(), msg_link_log.clone().to_string());
    }
    else                                                                                                            // Autorizzazione non concessa
    {
        println!("\n---------------------------------------");
        println!("\nOPERAZIONE SUI DATI: ACCESSO\n");
        println!("- Non è possibile eseguire l'operazione di accesso per mancanza di autorizzazione!");
    }

    Ok(())
}