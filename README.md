# k8s Fagdag

## Forberedelser
Sørg for å ha installert følgende, man kan finne detaljerte instrukser for begge på deres nettsider
* [minikube](https://minikube.sigs.k8s.io/docs/start/)
* [kubectl](https://kubernetes.io/docs/tasks/tools/install-kubectl/)
* I tilegg er en editor for å redigere yaml filer og en god terminal anbefalt.
* Hvis du kjører på OSX eller Linux er `watch` også nyttig

Underveis kan det være nyttig å se status på cluster (for de som ikke liker terminaler) og to gode alternativer er 
kommandoen `minikube dashboard` eller [k9s](https://github.com/derailed/k9s).


**Husk å sjekke at alt fungerer, helst før fagdagen er i gang. Trenger man hjelp i dagene før fagdagen kom innom #fagdag på Slack.**

## Feilsøking
### Problem: Kubectl får ikke kontakt med minikube
F.eks. `kubectl get pods` returnerer connection feil
#### Feilsøking 1:
Sjekk at minikube kjører som det skal (er alt running?): `minikube status`

#### Feilsøking 2:
Test at du har riktig context: `kubectl config get-contexts` . Det skal vise "minikube" som context (det er clusteret du skal være koblet til).

#### Feilsøking 3:
Sjekk hvilken versjon av minikube, kubectl og docker du har og at det ser riktig ut:
- `kubectl version`
- `minikube version`
- `docker version`

#### Feilsøking 4: 
Det kan være du må slette eksisterende cluster hvis du har lekt med minikube før: 
`minikube stop` hvis det kjører, så `minikube delete` og så start på nytt `minikube start`

### Problem: Kubectl får unauthorized når du kjører kommandoer
#### Feilsøking 1:
Sjekk at du har riktig contect ved å kjøre `kubectl config get-contexts`

Minkube skal være den contexten du kjøre mot.

#### Feilsøking 2:
Hvis du har oppdatert minikube så kan context ha blitt korrupt og du må slette contexts. OBS: du vil miste andre contexts også når du sletter disse.

- Stop minikube `minikube stop`
- Delete minikube `minikube delete`
- Slett .minikube og .kube mappen i din "home"-katalog. 
- Start minikube `minikube start`
- Oppdater context `minikube update-context`

### Problem: minikube vil ikke starte skikkelig
Dette har vi f.eks. opplevd etter at vi har oppdatert minikube fra en eldre versjon.

#### Feilsøking 1:
Sjekk om det kommer noen hint når du kjører `minikube start` eller `minikube status`.
Det kan komme løsningsforslag der :) 

#### Feilsøking 2:
Sjekk hvilken versjon av minikube, kubectl og docker du har og at det ser riktig ut:
- `kubectl version`
- `minikube version`
- `docker version`

#### Feilsøking 3:
Det kan være du må slette eksisterende cluster hvis du har lekt med minikube før:
`minikube stop` hvis det kjører, så `minikube delete` og så start på nytt `minikube start`

**Hvis man trenger hjelp, spør oss gjerne på #fagdag på Slack. Vi er ikke eksperter men er veldig gode på å Google :)**
