## Oppgave1: 
TODO Forklare hva denne oppgaven går ut på og hva man skal gjøre.


Brukerne skal lære apply deployment. 
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

#### Kubernetes tutorial
Man kan også leke med tilsvarende funksjonalitet i tutorials hos https://kubernetes.io:

**Deploy:**
https://kubernetes.io/docs/tutorials/kubernetes-basics/deploy-app/deploy-intro/

**Explore:**
https://kubernetes.io/docs/tutorials/kubernetes-basics/explore/

