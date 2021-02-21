## Oppgave1: 


Så kjøre kubectl kommandoer for å se hva som kjører og blir litt kjent med kubectl

`kubectl apply -f`

`kubectl get pods`

`kubectl get services`

Describe kommando for å vise kjørende, pod, service

`kubectl describe pod ...`

`kubectl describe service ...`


Hente kjørende deploy (kan også gjøres for service, ingress osv)
`kubectl get deploy deploymentname -o yaml --export
`
