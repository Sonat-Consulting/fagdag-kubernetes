## Oppgave4: 

I denne oppgaven skal vi se på muligheten for at forskjellige app'er og seriver kan snakke
sammen internt i Kubernetes-clusteret.

del 1 - Kjøre opp to forskjellige "Apper"
-------

Frem til nå, så har vi bare hart en "app" kjørende.. 

Nå skal vi late som om vi har to forskjellige app'er.

Siden vi bare har en faktisk app, så kan vi deploye den flere ganger.

Vi begynner med å fjerne alt vi hadde fra forrige oppgaver:

    kubectl delete -f ../oppgave-3/deployment-6.yml

Du kan sjekke at alt er borte vekk med:

    kubectl get pods
    kubectl get deployments
    kubectl get services
    kubectl get ingresses

Da er vi klare til å deploy den første app'en ``green``.
Hvis du studerer filen [deployment-green-1.yml](deployment-green-1.yml) 
så ser du at denne mountes i lastbalansereren under ``/green``.

Deploy appen ``green``:

    kubectl apply -f deployment-green-1.yml

Siden vi nå skal mounte flere app'er i lastbalansereren har vi flyttet Ingress-informajsonen til
en egen fil: [deployment-ingress-1.yml](deployment-ingress-1.yml).

Deploy denne:

    kubectl apply -f deployment-ingress-1.yml

Du kan nå se at Green-appen er tilgjengelig under ``/green`` i browseren.

Da er vi klare til å deploye ``blue``:

    kubectl apply -f deployment-blue-1.yml

Vi må også configurere Ingress til å mappe ``/blue``:

    kubectl apply -f deployment-ingress-2.yml


Sä dër... Da har vi to apper kjørende på to forskjellige url'er i Kubernetes.. 

Sjekk gjerne både ``/green`` og ``/blue`` i nettleseren..

Kult, ikke sant?

Del 2 - Få appene til å snakke sammen
-----

Hensikten med denne Del 2 er å late om at ``Green`` må kalle ``Blue`` for å virke..


Hvis du ser i [deployment-green-2.yml](deployment-green-2.yml) så ser du at vi har confet:

    - name: REMOTE_SERVICE
      value: "http://demo-service-blue.default:7070/info"

Her bruker vi den interne DNS-resolvingen i Kubernetes. DNS-navnet er **service-navn** - 
etterfulgt av **.namespace**.

I disse oppgavene har vi bare jobbet med default-namespacet, men en typisk bruk av namespace, kan være å skille
mellom forskjellige miljø og/eller forskjellige avdelinger / grupper av applikasjoner.


Vi deployer endring til **green** om at den skal snakke med **blue**:

    kubectl apply -f deployment-green-2.yml

Da er det bare å begynne å reloade ``/green`` i browseren mens vi venter på at endringene skal bli synlige..

Når ny **green** er synlig, vil du se at den snakker med **blue** under ``one of my friends``.

Her skal du se at den snakker med ``demo-app-blue-****`` og antall requester den har registrert..
Lek med å gå mot `/blue` og `/green`

Dere er faktisk vitne til amazing stuff ;)

EOF
