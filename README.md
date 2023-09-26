# Applicazione decentralizzata
Le <strong>applicazioni decentralizzate</strong> sono le <strong>applicazioni sviluppate dagli utenti consumatori dell’ecosistema per raccogliere ed elaborare i dati degli utenti produttori</strong>. Queste applicazioni possono essere sviluppate per diversi dispositivi e possono offrire svariate funzionalità. Nonostante ciò, le applicazioni decentralizzate condividono sempre due moduli: <strong>Raccolta</strong> ed <strong>Elaborazione</strong>. 
<br>Come suggeriscono i nomi, il primo è impiegato nelle operazioni di raccolta dati; mentre il secondo è dedicato alla loro elaborazione. Siccome le applicazioni decentralizzate possono essere di varie tipologie, le modalità di raccolta ed elaborazione possibili diventano molteplici ed eterogenee. Pertanto, questi moduli non sono stati approfonditi nel dettaglio a livello di componenti interni. Esistono però varie operazioni che sono comuni alle varie implementazioni dei moduli. Innanzitutto, prima di eseguire un’operazione sui dati, bisogna controllare che l’utente produttore loggato abbia effettivamente concesso l’autorizzazione. Questo controllo viene fatto interrogando lo ISC Autorizzazioni tramite la Wasm-cli. In caso di esito positivo, bisognerà recuperare i riferimenti dei canali e l’oggetto Author, però dallo smart contract GestioneApplicazioni. Solo allora sarà possibile eseguire l’operazione di raccolta ed elaborazione. Per la raccolta, i dati saranno scritti sul canale dati o nel file system IPFS. Al contrario, l’elaborazione comporterà la lettura da queste due reti. Ogni operazione eseguita con successo comporterà anche la sua registrazione sul canale log.

## Moduli software
L'applicazione decentralizzata è composta da diversi moduli Rust:
- [streams.rs](https://github.com/Tesi-Magistrale-FP/applicazione_decentralizzata/blob/main/src/moduli/streams.rs): gestisce il controllo delle autorizzazioni e i riferimenti dei canali dati e log, con i relativi autori e iscritti:
  - <strong>Controlla autorizzazione:</strong> controlla la concessione dell'autorizzazione per un consumatore da parte di un produttore.
  - <strong>Crea canali:</strong> memorizza sullo smart contract i riferimenti per accedere ai canali e i relativi oggetti Author.
  - <strong>Modifica indirizzi ultimi messaggi:</strong> aggiorna gli indirizzi che puntano agli ultimi messaggi scritti sul canale dati e log.
  - <strong>Aggiungi iscritto:</strong> memorizza sullo smart contract gli oggetti Subscriber associati a dei consumatori che hanno ricevuto l'autorizzazione per accedere ai canali.
  - <strong>Indirizzi ultimi messaggi:</strong> ottiene gli indirizzi che puntano agli ultimi messaggi scritti sul canale dati e log.
  - <strong>Ottiene autore:</strong> ottiene gli oggetti Author usati da un produttore in un'operazione sui dati.
  - <strong>Ottiene iscritto:</strong> ottiene gli oggetti Subscriber usati da un consumatore in un'operazione sui dati.
- [raccolta.rs](https://github.com/Tesi-Magistrale-FP/applicazione_decentralizzata/blob/main/src/moduli/raccolta.rs): a:
  - <strong>A:</strong> a.
- [elaborazione.rs](https://github.com/Tesi-Magistrale-FP/applicazione_decentralizzata/blob/main/src/moduli/elaborazione.rs): a:
  - <strong>A:</strong> a.