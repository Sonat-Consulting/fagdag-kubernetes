temp forklaring


Ingress, skikkelig normal round robin lastbalanserer , fungerer ikke standardedn
der minikube kjører i docker.

løsning:

kjør dette - hos meg fungerer dette mye raskere også:

    minikube config set vm-driver hyperkit
    minikube delete
    minikube start
    minikube addons enable ingress

så kan man kjøre opp dashboard hvis man vil:

    minikube dashboard


Hvis man står i denne mappen, så kan man kjøre opp:

    kubectl apply -f deployment.yml


Da starter den opp 2x pod på port 8080.. så lager den service som er på port 7070 fremfor dem.

Så starter den ingress lastablanserer som du kan nå i browser.
for å finne ip'en kjører du:

    kubectl get ingress

porten er 80.


da får du ny instance av pod hver gang du reloader.

funker som en kule..

