use anyhow::Result;
use core::str::FromStr;
use rand::Rng;
use iota_streams::{
    app::transport::tangle::client::Client,
    app_channels::api::tangle::{Address, Author, Bytes, ChannelType, Subscriber},
};

mod moduli;

const ALPH9: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ9";
const URL_DEVNET: &str = "https://api.lb-0.h.chrysalis-devnet.iota.cafe/";

static mut AUTORE_DATI: Option<Author<Client>> = None;
static mut ANNUNCIO_DATI: Option<String> = None;
static mut IND_ULT_MSG_DATI: Option<String> = None;
static mut ISCRITTO_DATI: Option<Subscriber<Client>> = None;

static mut AUTORE_LOG: Option<Author<Client>> = None;
static mut ANNUNCIO_LOG: Option<String> = None;
static mut IND_ULT_MSG_LOG: Option<String> = None;
static mut ISCRITTO_LOG: Option<Subscriber<Client>> = None;

#[tokio::main]
async fn main() -> Result<()> 
{
    let did_produttore: String = String::from("did:iota:2CzNQW4DLxEdpwBXJ6X3WW361oXTD787EuXpdF8A9oZJ");
    let did_consumatore: String = String::from("did:iota:8dQAzVbbf6FLW9ckwyCBnKmcMGcUV9LYJoXtgQkHcNQy");
    let id_app: i32 = 0;
    let id_op: i32 = 0;

    inizializzazione_canali().await?;    

    // Test modulo Streams
    let _esito_controllo: bool = moduli::streams::controlla_autorizzazione(id_app, id_op, did_produttore.clone(), did_consumatore.clone());
    unsafe 
    {    
        let _creazione_canali: bool = moduli::streams::crea_canali(id_app, id_op, did_produttore.clone(), AUTORE_DATI.as_mut().unwrap().to_string(), ANNUNCIO_DATI.as_mut().unwrap().to_string(), IND_ULT_MSG_DATI.as_mut().unwrap().to_string(), AUTORE_LOG.as_mut().unwrap().to_string(), ANNUNCIO_LOG.as_mut().unwrap().to_string(), IND_ULT_MSG_LOG.as_mut().unwrap().to_string());
        let _modifica_indirizzi: bool = moduli::streams::modifica_ind_ult_msg(id_app, id_op, did_produttore.clone(), IND_ULT_MSG_DATI.as_mut().unwrap().to_string(), IND_ULT_MSG_LOG.as_mut().unwrap().to_string());
        let _aggiunta_iscritti: bool = moduli::streams::aggiungi_iscritto(id_app, id_op, did_produttore.clone(), did_consumatore.clone(), ISCRITTO_DATI.as_mut().unwrap().to_string(), ISCRITTO_LOG.as_mut().unwrap().to_string());
        let _ind_ult_msg: Vec<String> = moduli::streams::ind_ult_msg(id_app, id_op, did_produttore.clone());
        let _autori: Vec<String> = moduli::streams::ottieni_autore(id_app, id_op, did_produttore.clone());
        let _iscritti: Vec<String> = moduli::streams::ottieni_iscritto(id_app, id_op, did_produttore.clone(), did_consumatore.clone());
    }
    
    Ok(())
}

async fn inizializzazione_canali() -> iota_streams::core::Result<()>
{
    println!("\n---------------------------------------");
    println!("\nINIZIALIZZAZIONE CANALI\n");

    // -------------------------------------------------
    // Lato Author -> Creazione dell'Author e del canale 
    // -------------------------------------------------

    // Generazione seed per il canale dati
    let seed_dati: &str = &(0..81)
		.map(|_| {
            ALPH9
                .chars()
                .nth(rand::thread_rng().gen_range(0..27))
                .unwrap()
        })
        .collect::<String>();

    // Generazione seed per il canale log
    let seed_log: &str = &(0..81)
		.map(|_| {
            ALPH9
                .chars()
                .nth(rand::thread_rng().gen_range(0..27))
                .unwrap()
        })
        .collect::<String>();

    println!("- Seed generati");

    let client = Client::new_from_url(URL_DEVNET);                                                          // Creazione di un Transport Client

    unsafe
    {
        // Generazione Author del produttore per i canali dati e log
        AUTORE_DATI = Some(Author::new(seed_dati, ChannelType::SingleBranch, client.clone()));
        AUTORE_LOG = Some(Author::new(seed_log, ChannelType::SingleBranch, client.clone()));

        println!("- Autori generati");

        // Ottenimento dei link ai messaggi di annuncio dei canali dati e log
        let link_annuncio_dati = AUTORE_DATI.as_mut().unwrap().send_announce().await?;
        ANNUNCIO_DATI = Some(link_annuncio_dati.to_string());

        let link_annuncio_log = AUTORE_LOG.as_mut().unwrap().send_announce().await?;
        ANNUNCIO_LOG = Some(link_annuncio_log.to_string());

        println!("- Link ai messaggi di annuncio ottenuti");

        // ---------------------------------------------------------------------------
        // Lato Subscriber -> Creazione del Subscriber e relativa iscrizione al canale
        // ---------------------------------------------------------------------------

        // Generazione Subscriber del consumatore per i canali dati e log 
        ISCRITTO_DATI = Some(Subscriber::new("Subscriber dati", client.clone()));
        ISCRITTO_LOG = Some(Subscriber::new("Subscriber log", client.clone()));

        println!("- Iscritti generati");

        // Invio della richiesta per iscriversi ai canali dati e log
        let indirizzo_annuncio_dati = Address::from_str(&ANNUNCIO_DATI.as_mut().unwrap())?;
        let indirizzo_annuncio_log = Address::from_str(&ANNUNCIO_LOG.as_mut().unwrap())?;

        ISCRITTO_DATI.as_mut().unwrap().receive_announcement(&indirizzo_annuncio_dati).await?;
        ISCRITTO_LOG.as_mut().unwrap().receive_announcement(&indirizzo_annuncio_log).await?;

        println!("- Link di annuncio ricevuto");

        let richiesta_iscrizione_dati = ISCRITTO_DATI.as_mut().unwrap().send_subscribe(&indirizzo_annuncio_dati).await?;
        let richiesta_iscrizione_log = ISCRITTO_LOG.as_mut().unwrap().send_subscribe(&indirizzo_annuncio_log).await?;

        let richiesta_iscrizione_string_dati = richiesta_iscrizione_dati.to_string();
        let richiesta_iscrizione_string_log = richiesta_iscrizione_log.to_string();

        println!("- Richieste di iscrizione inviate");

        // ---------------------------------------------------------------------------------------
        // Lato Author -> Conferma iscrizione del Subscriber al canale e pubblicazione dei messaggi
        // ---------------------------------------------------------------------------------------

        // Recupero degli indirizzi delle richieste di iscrizione
        let indirizzo_richiesta_dati = Address::from_str(&richiesta_iscrizione_string_dati)?;
        let indirizzo_richiesta_log = Address::from_str(&richiesta_iscrizione_string_log)?;                                                  

        // Recupero e accettazione delle richieste di iscrizione
        AUTORE_DATI.as_mut().unwrap().receive_subscribe(&indirizzo_richiesta_dati).await?;
        AUTORE_LOG.as_mut().unwrap().receive_subscribe(&indirizzo_richiesta_log).await?;

        println!("- Richieste di iscrizione ricevute ed elaborate");

        // Pubblicazione messaggio keyload per consentire l'accesso ai canali dati e log ai consumatori
        let (link_messaggio_keyload_dati, _seq) = AUTORE_DATI.as_mut().unwrap().send_keyload_for_everyone(&link_annuncio_dati).await?;
        let (link_messaggio_keyload_log, _seq) = AUTORE_LOG.as_mut().unwrap().send_keyload_for_everyone(&link_annuncio_log).await?;
        
        // Memorizzazione indirizzo degli ultimi messaggi pubblicati sui canali dati e log
        IND_ULT_MSG_DATI = Some(link_messaggio_keyload_dati.to_string());
        IND_ULT_MSG_LOG = Some(link_messaggio_keyload_log.to_string());

        println!("- Iscritti al canale con successo");
    }

    Ok(())
}