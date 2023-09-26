use std::process::Command;
use std::process::Stdio;

// Controllo rilascio autorizzazione per un consumatore da parte di un produttore
pub fn controlla_autorizzazione(id_app: i32, id_op: i32, did_produttore: String, did_consumatore: String) -> bool											
{
    let mut esito: bool = true;

    println!("\n---------------------------------------");
    println!("\nCONTROLLA AUTORIZZAZIONE\n");

    println!("- DID produttore: {}\n- DID consumatore: {}\n- ID applicazione: {}\n- ID operazione: {}", did_produttore, did_consumatore, id_app, id_op);

    // Esegue il comando della wasp-cli per controllare la concessione dell'autorizzazione dallo ISC Autorizzazioni
    let cmd_view = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["chain", "call-view", "autorizzazioni", "controllaAutorizzazione", "String", "idApplicazione", "Int32", &id_app.to_string(), "String", "idOperazione", "Int32", &id_op.to_string(), "String", "didConsumatore", "String", &did_consumatore, "String", "didProduttore", "String", &did_produttore, "--chain=mychain"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let cmd_res = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["decode", "string", "esitoC", "bool"])
        .stdin(Stdio::from(cmd_view.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let output_view = cmd_res.wait_with_output().unwrap();

    // Ottiene l'output del comando eseguito e lo elabora per capire l'esito del controllo
    let output_s: String = String::from_utf8(output_view.stdout).unwrap();

    if ! output_s.contains("true") && ! output_s.contains("false") {
        println!("- Errore controllo autorizzazione -> {output_s}");
        esito = false;
    }
    else if output_s.contains("false") {
        println!("- Autorizzazione non concessa!");
        esito = false;
    }
    else if output_s.contains("true") {
        println!("- Autorizzazione concessa!");
    }

    return esito;
}

// Salva i riferimenti al canale dati e log, assieme ai rispettivi autori
pub fn crea_canali(id_app: i32, id_op: i32, did_produttore: String, autore_dati: String, annuncio_dati: String, ind_ult_msg_dati: String, autore_log: String, annuncio_log: String, ind_ult_msg_log: String) -> bool
{
    let mut esito = true;                                                                                 			// Esito dell'operazione di pubblicazione
    
    println!("\n---------------------------------------");
    println!("\nCREAZIONE CANALI DATI E LOG\n");

    // Mostra i dati passati
    println!("- ID applicazione: {}\n- ID operazione: {}\n- DID produttore: {}\n- Autore canale dati: {}\n- Link annuncio canale dati: {}\n- Indirizzo ultimo messaggio sul canale dati: {}\n- Autore canale log: {}\n- Link annuncio canale log: {}\n- Indirizzo ultimo messaggio sul canale log: {}", id_app, id_op, did_produttore, autore_dati, annuncio_dati, ind_ult_msg_dati, autore_log, annuncio_log, ind_ult_msg_log);

    // Esegue il comando della wasp-cli per eseguire salvare i riferimenti dei canali dati e log e dei relativi autori sullo ISC GestioneApplicazioni
    let mut output = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["chain", "post-request", "gestioneapplicazioni", "creaCanali", "String", "idApp", "Int32", &id_app.to_string(), "String", "idOp", "Int32", &id_op.to_string(), "String", "didProduttore", "String", &did_produttore, "String", "autoreDati", "String", &autore_dati, "String", "annuncioDati", "String", &annuncio_dati, "String", "indUltMsgDati", "String", &ind_ult_msg_dati, "String", "autoreLog", "String", &autore_log, "String", "annuncioLog", "String", &annuncio_log, "String", "indUltMsgLog", "String", &ind_ult_msg_log, "--chain=mychain", "-s"])
        .output()
        .unwrap();

    // Ottiene l'output del comando eseguito e lo elabora per capire l'esito della creazione
    let mut output_s: String = String::from_utf8(output.stdout).unwrap();

    if ! output_s.contains("Waiting for tx requests to be processed...")                                            // Errore esecuzione comando per chiamare la funzione 
    {
        println!("- Errore creazione canali -> {output_s}");
        esito = false;
    }
    else                                                                                                          	// Chiamata alla funzione avvenuta con successo
    {
        // Recupera l'indirizzo della transazione usata per chiamare la funzione
        let output_elab: String = output_s.replace("Waiting for tx requests to be processed...", "");
        let ind_trans: String = String::from(output_elab.split_whitespace().last().unwrap().replace(")", ""));
        
        // Recupera l'evento generato dalla funzione per comprendere l'esito della creazione
        output = Command::new("wasp-cli")
            .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
            .args(["chain", "request", &ind_trans])
            .output()
            .unwrap();

        output_s = String::from_utf8(output.stdout).unwrap();

        if output_s.contains("gestioneapplicazioni.canaliCreati")                                       	        // Creazione avvenuta con successo
        {
            println!("- Creazione avvenuta!");
        }
        else                                                                                            			// Creazione non avvenuta
        {
            println!("- Creazione non avvenuta!");
            esito = false;
        }
    }

    return esito;
}

// Aggiorna gli indirizzi degli ultimi messaggi pubblicati sui canali dati e log
pub fn modifica_ind_ult_msg(id_app: i32, id_op: i32, did_produttore: String, ind_ult_msg_dati: String, ind_ult_msg_log: String) -> bool
{
    let mut esito = true;                                                                                 			// Esito dell'operazione di pubblicazione
    
    println!("\n---------------------------------------");
    println!("\nAGGIORNAMENTO INDIRIZZI ULTIMI MESSAGGI\n");

    // Mostra i dati passati
    println!("- ID applicazione: {}\n- ID operazione: {}\n- DID produttore: {}\n- Nuovo indirizzo ultimo messaggio sul canale dati: {}\n- Nuovo indirizzo ultimo messaggio sul canale log: {}", id_app, id_op, did_produttore, ind_ult_msg_dati, ind_ult_msg_log);

    // Esegue il comando della wasp-cli per eseguire aggiornare gli indirizzi sullo ISC GestioneApplicazioni
    let mut output = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["chain", "post-request", "gestioneapplicazioni", "modificaIndUltMsg", "String", "idApp", "Int32", &id_app.to_string(), "String", "idOp", "Int32", &id_op.to_string(), "String", "didProduttore", "String", &did_produttore, "String", "indUltMsgDati", "String", &ind_ult_msg_dati, "String", "indUltMsgLog", "String", &ind_ult_msg_log, "--chain=mychain", "-s"])
        .output()
        .unwrap();

    // Ottiene l'output del comando eseguito e lo elabora per capire l'esito dell'aggiornamento
    let mut output_s: String = String::from_utf8(output.stdout).unwrap();

    if ! output_s.contains("Waiting for tx requests to be processed...")                                            // Errore esecuzione comando per chiamare la funzione 
    {
        println!("- Errore aggiornamento indirizzi -> {output_s}");
        esito = false;
    }
    else                                                                                                          	// Chiamata alla funzione avvenuta con successo
    {
        // Recupera l'indirizzo della transazione usata per chiamare la funzione
        let output_elab: String = output_s.replace("Waiting for tx requests to be processed...", "");
        let ind_trans: String = String::from(output_elab.split_whitespace().last().unwrap().replace(")", ""));
        
        // Recupera l'evento generato dalla funzione per comprendere l'esito dell'aggiornamento
        output = Command::new("wasp-cli")
            .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
            .args(["chain", "request", &ind_trans])
            .output()
            .unwrap();

        output_s = String::from_utf8(output.stdout).unwrap();

        if output_s.contains("gestioneapplicazioni.indirizziModificati")                                       	    // Aggiornamento riuscito con successo
        {
            println!("- Aggiornamento riuscito!");
        }
        else                                                                                            			// Aggiornamento non riuscito
        {
            println!("- Aggiornamento non riuscito!");
            esito = false;
        }
    }

    return esito;
}

// Salva i riferimenti degli iscritti del canale dati e log
pub fn aggiungi_iscritto(id_app: i32, id_op: i32, did_produttore: String, did_consumatore: String, iscritto_dati: String, iscritto_log: String) -> bool
{
    let mut esito = true;                                                                                 			// Esito dell'operazione di pubblicazione
    
    println!("\n---------------------------------------");
    println!("\nSALVATAGGIO ISCRITTI\n");

    // Mostra i dati passati
    println!("- ID applicazione: {}\n- ID operazione: {}\n- DID produttore: {}\n- DID consumatore: {}\n- Iscritto canale dati: {}\n- Iscritto canale log: {}", id_app, id_op, did_produttore, did_consumatore, iscritto_dati, iscritto_log);

    // Esegue il comando della wasp-cli per eseguire il salvataggio degli iscritti sullo ISC GestioneApplicazioni
    let mut output = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["chain", "post-request", "gestioneapplicazioni", "aggiungiIscritto", "String", "idApp", "Int32", &id_app.to_string(), "String", "idOp", "Int32", &id_op.to_string(), "String", "didProduttore", "String", &did_produttore, "String", "didConsumatore", "String", &did_consumatore, "String", "iscrittoDati", "String", &iscritto_dati, "String", "iscrittoLog", "String", &iscritto_log, "--chain=mychain", "-s"])
        .output()
        .unwrap();

    // Ottiene l'output del comando eseguito e lo elabora per capire l'esito del salvataggio
    let mut output_s: String = String::from_utf8(output.stdout).unwrap();

    if ! output_s.contains("Waiting for tx requests to be processed...")                                            // Errore esecuzione comando per chiamare la funzione 
    {
        println!("- Errore aggiunta iscritto -> {output_s}");
        esito = false;
    }
    else                                                                                                          	// Chiamata alla funzione avvenuta con successo
    {
        // Recupera l'indirizzo della transazione usata per chiamare la funzione
        let output_elab: String = output_s.replace("Waiting for tx requests to be processed...", "");
        let ind_trans: String = String::from(output_elab.split_whitespace().last().unwrap().replace(")", ""));
        
        // Recupera l'evento generato dalla funzione per comprendere l'esito del salvataggio
        output = Command::new("wasp-cli")
            .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
            .args(["chain", "request", &ind_trans])
            .output()
            .unwrap();

        output_s = String::from_utf8(output.stdout).unwrap();

        if output_s.contains("gestioneapplicazioni.iscrittoAggiunto")                                       	    // Aggiunta avvenuta non successo
        {
            println!("- Iscritti aggiunti!");
        }
        else                                                                                            			// Aggiunta non riuscita
        {
            println!("- Iscritti non aggiunti!");
            esito = false;
        }
    }

    return esito;
}

// Restituisce gli indirizzi degli ultimi messaggi pubblicati sul canale dati e log
pub fn ind_ult_msg(id_app: i32, id_op: i32, did_produttore: String) -> Vec<String>											
{
    let mut esito: Vec<String> = Vec::new();

    println!("\n---------------------------------------");
    println!("\nINDIRIZZI ULTIMI MESSAGGI\n");

    println!("- ID applicazione {}\n- ID operazione {}\n- DID produttore {}", id_app, id_op, did_produttore);

    // Esegue il comando della wasp-cli per eseguire ottenere gli indirizzi degli ultimi messaggi dallo ISC GestioneApplicazioni
    let cmd_view = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["chain", "call-view", "gestioneapplicazioni", "indUltMsg", "String", "idApp", "Int32", &id_app.to_string(), "String", "idOp", "Int32", &id_op.to_string(), "String", "didProduttore", "String", &did_produttore, "--chain=mychain"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let cmd_res = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["decode", "string", "indirizzi", "string"])
        .stdin(Stdio::from(cmd_view.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let output_view = cmd_res.wait_with_output().unwrap();

    // Ottiene l'output del comando eseguito e lo elabora per ottenere gli indirizzi degli ultimi messaggi
    let output_s: String = String::from_utf8(output_view.stdout).unwrap().replace("indirizzi: ", "").replace("\"", "").replace("\n", "");

    if output_s.len() == 0 																							// Nessun indirizzo presente
    {
        println!("- Nessun indirizzo presente!");
    }
    else																											// Indirizzi recuperati
    {
        let indirizzi: Vec<&str> = output_s.split("|").collect();													// Ottiene i due indirizzi separati dal carattere |
        
        println!("- Indirizzo ultimo messaggio canale dati: {}\n- Indirizzo ultimo messaggio canale log: {}", indirizzi[0], indirizzi[1]);

        esito.push(indirizzi[0].to_string());
        esito.push(indirizzi[1].to_string());
    }

    return esito;
}

// Restituisce gli autori dei canali dati e log associati a un'operazione di un'applicazione
pub fn ottieni_autore(id_app: i32, id_op: i32, did_produttore: String) -> Vec<String>											
{
    let mut esito: Vec<String> = Vec::new();

    println!("\n---------------------------------------");
    println!("\nOTTIENI AUTORE\n");

    println!("- ID applicazione {}\n- ID operazione {}\n- DID produttore {}", id_app, id_op, did_produttore);

    // Esegue il comando della wasp-cli per eseguire ottenere gli autori dallo ISC GestioneApplicazioni
    let cmd_view = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["chain", "call-view", "gestioneapplicazioni", "ottieniAutore", "String", "idApp", "Int32", &id_app.to_string(), "String", "idOp", "Int32", &id_op.to_string(), "String", "didProduttore", "String", &did_produttore, "--chain=mychain"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

        let cmd_res = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["decode", "string", "autori", "string"])
        .stdin(Stdio::from(cmd_view.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let output_view = cmd_res.wait_with_output().unwrap();

    // Ottiene l'output del comando eseguito e lo elabora per ottenere gli autori
    let output_s: String = String::from_utf8(output_view.stdout).unwrap().replace("autori: ", "").replace("\"", "").replace("\n", "");

    if output_s.len() == 0 																							// Nessun autore presente
    {
        println!("- Nessun autore presente!");
    }
    else																											// Autori recuperati
    {
        let autori: Vec<&str> = output_s.split("|").collect();													    // Ottiene i due autori separati dal carattere |
        
        println!("- Autore canale dati: {}\n- Autore canale log: {}", autori[0], autori[1]);

        esito.push(autori[0].to_string());
        esito.push(autori[1].to_string());
    }

    return esito
}

// Restituisce gli iscritti dei canali dati e log associati a un'operazione di un'applicazione
pub fn ottieni_iscritto(id_app: i32, id_op: i32, did_produttore: String, did_consumatore: String) -> Vec<String>											
{
    let mut esito: Vec<String> = Vec::new();

    println!("\n---------------------------------------");
    println!("\nOTTIENI ISCRITTO\n");

    println!("- ID applicazione {}\n- ID operazione {}\n- DID produttore {}\n- DID consumatore {}", id_app, id_op, did_produttore, did_consumatore);

    // Esegue il comando della wasp-cli per eseguire ottenere gli iscritti dallo ISC GestioneApplicazioni
    let cmd_view = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["chain", "call-view", "gestioneapplicazioni", "ottieniIscritto", "String", "idApp", "Int32", &id_app.to_string(), "String", "idOp", "Int32", &id_op.to_string(), "String", "didProduttore", "String", &did_produttore, "String", "didConsumatore", "String", &did_consumatore, "--chain=mychain"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

        let cmd_res = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["decode", "string", "iscritti", "string"])
        .stdin(Stdio::from(cmd_view.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let output_view = cmd_res.wait_with_output().unwrap();

    // Ottiene l'output del comando eseguito e lo elabora per ottenere gli iscritti
    let output_s: String = String::from_utf8(output_view.stdout).unwrap().replace("iscritti: ", "").replace("\"", "").replace("\n", "");

    if output_s.len() == 0 																							// Nessun iscritto presente
    {
        println!("- Nessun iscritto presente!");
    }
    else																											// Iscritti recuperati
    {
        let iscritti: Vec<&str> = output_s.split("|").collect();													// Ottiene i due iscritti separati dal carattere |
        
        println!("- Iscritto canale dati: {}\n- Iscritti canale log: {}", iscritti[0], iscritti[1]);

        esito.push(iscritti[0].to_string());
        esito.push(iscritti[1].to_string());
    }

    return esito
}