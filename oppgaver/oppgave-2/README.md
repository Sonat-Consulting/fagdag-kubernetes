## Oppgave 2:

I denne oppgaven skal dere bli kjent med hva som skjer når en pod restarter eller krasjer. 
Dere skal også bli kjent med hvordan Kubernetes vurderer om en pod trenger å restartes.

Del 1
---------

Fra forrige oppgave så har vi flere POD'er som alle henger sammen i en deployment.

    kubectl get pod
    kubectl get deployment

Deploymenten forteller Kubernetes at det til en hver tid skal være 2 (Eller den verdien som ``replicas`` har) Poder tilgjengelig.

Tips: For å **live** se hva som skjer med pod'ene, kan du kjøre følgende i et terminal-vindu:
    
    watch kubectl get pods

Du kan prøve å fjerne en pod og se hva som skjer:

    kubectl delete pod demo-app-7d4b795879-c4qqh

Du kan også få applikasjonen som kjører inni pod'en til å krasje med å bruke ``port-forward`` og 
gå til ``http://localhost:8080/crash``

Hva skjer med poden/ene?

Del 2
-------

En ting er når en pod crash'er... Men hva om den fortsatt kjører, men begynner å oppføre seg dårlig??
Siden app'en vår inneholder en helsesjekk så kan vi fortelle Kubernetes å bruke denne

Detaljer info om dette finnes [her](https://kubernetes.io/docs/tasks/configure-pod-container/configure-liveness-readiness-startup-probes/)


I filen [deployment-4.yml](deployment-4.yml) har vi lagt til følgende:

    readinessProbe:
      httpGet:
        path: /health
        port: 8080 # I prod bør dette være annen port en hoved-port
      initialDelaySeconds: 5 # Venter før den begynner å sjekke om app'en er klar
      periodSeconds: 1 # Hvor ofte den sjekker
      failureThreshold: 5 # Hvor mange feil som skal til før den konkluderer..
    livenessProbe:
      httpGet:
        path: /health
        port: 8080 # I prod bør dette være annen port en hoved-port
      periodSeconds: 1 # Hvor ofte den sjekker
      failureThreshold: 5 # Hvor mange feil som skal til før den konkluderer..

Hvis du først starter følende i ett terminal-vindu:

    watch kubectl get pods

Så ser du bedre hva som skjer, hvis du applyer filen:

    kubectl apply -f deployment-4.yml


Hvis du vil få en av pod'ene våre til å bli **unhealthy**, så kan du bruke ``port-forward`` til å kalle
``http://localhost:8080/unhealthy``.

Da vil pod'en begynne å svare "FEIL" på helsesjekken og Kubernetes vil ta afære.. Følg med og se hva som skjer..

