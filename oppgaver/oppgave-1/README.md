Oppgave1
===========

Hensikten med denne oppgaven er å deploye noe til Kubernetes og eksprimentere med det.

Del 1
----------

Hvis ikke din lokale Kubernetes-instance allerede kjører, så må du starte den:

    minikube config set vm-driver hyperkit # default er VirtualBox
    minikube start
minikube addons enable ingress # Trengs bare å kjøre en gang

(Hvis du får problemer med å starte så kan du prøve en eller flere av disse kommandoene):

    minikube config set vm-driver hyperkit # default er VirtualBox - hyperkit har bedre Ip/Routing støtte
    minikube delete # Fjerner alt gammelt
    minikube start
    minikube addons enable ingress # Trengs bare å kjøre en gang

Vi kan minikube-status slik:

    minikube status

Nå fungerer forhåpentlig vis alt sammen ;)

Vi kan sjekke at det ikke at det ikke finnes **pod**er eller **deployments** i clusteret

    kubectl get pods
    kubectl get deployments

Nå skal vi deploye vår første pod. Studer deployment-filen [deployment-1.yml](deployment-1.yml) før vi går videre.

Nå skal vi fortelle Kubernetes at den skal deployes. For at dette skal fungere må du stå i denne mappen.

    kubectl apply -f deployment-1.yml

Nå kan vi se at pod'en vår starter opp:

    kubectl get pods

Denne pod'en er beskrevet i en deployment:

    kubectl get deployment

Vi kan sjekke log'en til pod'en vår - her må du bruke navn fra ``kubectl get pod``: 

    kubectl logs demo-app-7d4b795879-ntxwq

For å undersøke litt kan vi kobble oss på pod'en og starte et shell inni den.
Nå mu du bruke pod-navnet du fikk fra ``kubectl get pods`` slik som dette:

    kubectl exec -it demo-app-684d5f67dd-x5tss /bin/sh
    # nå kan du foreksempel kjøre følgende inni pod'en:
    ls
    export #for å se alle ENV-variablene
    #for å avslutte skriver du:
    exit

Appen som kjører i pod'en er en HTTP-server.. Vi kan kobble oss til den fra vår maskin slik:

    kubectl port-forward demo-app-684d5f67dd-x5tss 8080

Nå kan du åpne browser og gå til http://localhost:8080 - Reload gjerne siden mange ganger og se at ingenting endrer seg.. Alt treffer den samme pod'en.

Vi kan få masse info om pod'en vår:

    kubectl describe pod demo-app-684d5f67dd-x5tss

eller deployment'et vårt:

    kubectl describe deployment demo-app

Del 2
-------------

Til nå har vi bare hatt en enkelt pod.. Det er litt lite.. Vi ønsker flere.

I filen [deployment-2.yml](deployment-2.yml) (som er basert på den forrige filen) har jeg lagt inn ``replica: 2``.

Nå applyer vi denne slik:

    kubectl apply -f deployment-2.yml

Kjør ``kubectl get pods`` og se hva som skjedde.. prøv gjerne å endre antall replica og kjør ny apply. Lek litt ;)

Del 3
------------

Når vi nå har flere pod'er, så fungerer ``port-forward`` litt dårlig.

La oss "grupere" pod'ene våre i en Service og plasere en lastbalanserer (Ingress) fremfor slik at vi når de.

Ta en titt på filen [deployment-3.yml](deployment-3.yml) før du apply'er den.

    kubectl apply -f deployment-3.yml

Nå kan vi sjekke at vi har fått **servicer** og **Ingress** (Lastbalanserer)

    kubectl get service
    kubectl get ingress

Ingress er altså en lastbalanserer som fordeler trafikk til servicen og dermed pod'ene.
Typisk så vil man sørge for at ``http://www.din-bedrift.no`` termineres til denne Ingressen.

Nå skal vi teste den, og vi må kanskje vente litt før den er klar med en IP (ADDRESS):

    kubectl get ingress

Hos meg ser det slik ut:

    NAME         CLASS    HOSTS   ADDRESS        PORTS   AGE
    lb-service   <none>   *       192.168.64.2   80      104s

Da er det bare (for meg) å åpne browseren på ``http://192.168.64.2`` - du må bruke IPen du selv har..

Reload browseren og se at du treffer begge/alle PODene.. Prøv gjerne å endre antall ``replica`` og deploy på nytt.
Sjekk igjen at de nye treffes..


For å ta ned alt sammen kan vi kjøre

    kubectl delete -f deployment-3.yml

sjekk at alt er gone med

    kubectl get pods
    kubectl get service
    kubectl get deployment
    kubectl get ingress


Kjør det opp igjen ;)

    kubectl apply -f deployment-3.yml

----

Det var alt i denne oppgaven..