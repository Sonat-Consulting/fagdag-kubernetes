## Oppgave3:

I denne oppgaven skal vi undersøke hvordan rolling deployment fungerer i Kubernetes.

Litt forklaring
---

Som vi har beskrevet tidligere, så fungerer Kubernetes slik at vi forteller Kubernetes hvordan
vi vil at "verden skal se ut" - så sørger Kubernetes for at Verden blir slik.

Når vi snakker om å deploye ny versjon av en App i Kubernetes, så driter
egentlig Kubernetes i hva som er endret, enten det er versjonen av Docker-imaget, eller om det er config-endring..
Når vi applyer en ny deployment-beskrivelse, så sjekker Kubernetes om denne er forskjellig fra den eksisterende.
Hvis ja, så vil Kubernetes jobber for at dette blir den nye "sannheten".

Del 1 - Den nye versjonen virker ikke
----

Når vi deployer nye versjoner av en App i den virkelige verden,
så ønsker vi at Kundene våre ikke skal merke noe som helst..

La oss derfor "deploye" en versjon som ikke virker:

    kubectl apply -f deployment-5.yml

Hva skjer?

Sjekk med ``kubectl get pods`` eller følg med live med ``watch kubectl get pods``

Hva er galt?

Merker kunden problemet? Virker ting i nettleseren?

Klarer dere fikse problemet?

Del 2 - Deploye noe nytt som virker ;)
----

I [deployment-6.yml](deployment-6.yml) har vi har vi en config-endring der vi setter bakgrunds-fargen til noe annet.

Dere kan deplpoye denne endringen:

    kubectl apply -f deployment-6.yml

..og reloade app-siden via lastbalansereren som for meg er ``http://192.168.64.2/`` mens deployen kjører..

Hva skjer?

Eksprimenter med å endre farger og deploy endringene..




I Kubernetes er det ett fett om man endrer app-versjon - eller config.


endre bg-color i config-.. som er treg.

med watch k get pods så ser man at de nye pod'ene kommer opp..


kan også det i browser med å refreshe.



