## Oppgave 2:

I denne oppgaven skal dere bli kjent med hva som skjer når en pod restarter.
Man kan drepe en pod ved å kjøre `kubectl kill <pod-id>`
Da gir man beskjed til Kubernetes om å drepe den aktuelle pod-instansen og Kubernetes vil starte en ny.

Dere skal også se hvordan man kan se hva som har skjedd i Kubernetes clusteret ved å se på Kubernetes events: 
`kubectl get events`

### Ta ned pod
Kjør `kubectl kill <pod-id>`. 
Pod-id finner du ved å kjøre `kubectl get pods`
Hvordan oppførte applikasjonen seg mens den tok ned og Kubernetes starter opp ny pod?
Når du applikasjonen i nettleser mens den restarter?
Hvordan ser loggene ut etter restart? `kubectl logs <pod-id>`

Se hva som har skjedd i Kubernetes clusteret: `kubectl get events`

Sett gjerne ned antall replicas også til 1 og se hva som skjer når du dreper 1 pod.
Endre i yaml fil og deploy yaml fil på nytt ved `kubectl apply`

Du kan også få applikasjonen til å krasje ved å trykker på TODO

Er det forskjell fra når du tok den ned selv?

Bruk gjerne kommandoer og det du har lært fra forrige oppgave :)





