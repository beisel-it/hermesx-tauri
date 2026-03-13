# HermesX User Stories

## Epic 1: Time Tracking Core

**US-001** Als Nutzer möchte ich Mobiles Arbeiten in ZeusX per Tray-Klick starten, damit ich nicht jedes Mal den Browser öffnen muss.

**US-002** Als Nutzer möchte ich Mobiles Arbeiten per Tray-Klick beenden, damit das Ausstempeln keine Unterbrechung erfordert.

**US-003** Als Nutzer möchte ich eine Pause starten und beenden, damit Pausen korrekt in ZeusX erfasst werden.

**US-004** Als Nutzer möchte ich den aktuellen Arbeitsstatus (Arbeitend/Pause/Feierabend) im Systray-Icon sehen, damit ich immer informiert bin ohne die App zu öffnen.

**US-005** Als Nutzer möchte ich dass der Status nach App-Neustart wiederhergestellt wird, damit ich keine Daten verliere.

## Epic 2: Smarte Benachrichtigungen

**US-006** Als Nutzer möchte ich eine Erinnerung bekommen wenn ich vergessen habe einzustempeln (15min/30min nach Arbeitsstart), damit keine unbuchten Zeiten entstehen.

**US-007** Als Nutzer möchte ich beim Sperren des Bildschirms eine Warnung erhalten wenn ich noch eingestempelt bin, damit ich nicht vergesse auszustempeln.

**US-008** Als Nutzer möchte ich eine Benachrichtigung wenn meine Arbeitsstunden erfüllt sind, damit ich pünktlich Feierabend machen kann.

**US-009** Als Nutzer möchte ich eine Warnung wenn ich Überstunden mache (>30min über Solldauer), damit ich es bewusst entscheide.

**US-010** Als Nutzer möchte ich eine Erinnerung bei langer Inaktivität (>15min idle), damit Pausenzeiten korrekt gebucht werden.

**US-011** Als Nutzer möchte ich eine Erinnerung nach langem kontinuierlichem Arbeiten (>4h), damit ich regelmäßige Pausen einhalte.

**US-012** Als Nutzer möchte ich eine Erinnerung wenn meine Pause länger als geplant dauert, damit ich rechtzeitig zurückkomme.

## Epic 3: Notification Suppression

**US-013** Als Nutzer möchte ich Benachrichtigungen während Meetings unterdrücken, damit ich nicht gestört werde. (Meeting = Mikrofon aktiv / Zoom/Teams läuft)

**US-014** Als Nutzer möchte ich einen Quiet Mode aktivieren, damit ich alle Benachrichtigungen vorübergehend stumm schalten kann.

**US-015** Als Nutzer möchte ich Benachrichtigungen während Gaming unterdrücken (full-screen Spiele).

## Epic 4: Konfiguration

**US-016** Als Nutzer möchte ich Arbeitsbeginn und -dauer konfigurieren, damit die Erinnerungen zu meinem Zeitplan passen.

**US-017** Als Nutzer möchte ich Arbeitstage konfigurieren (Mo-Fr oder individuelle Tage), damit am Wochenende keine Erinnerungen kommen.

**US-018** Als Nutzer möchte ich Pausendauer und Inaktivitätsschwellen konfigurieren.

**US-019** Als Nutzer möchte ich ZeusX-Credentials sicher speichern (keychain), damit ich sie nicht jedes Mal eingeben muss.

**US-020** Als Nutzer möchte ich alle Einstellungen in einem übersichtlichen Settings-Fenster konfigurieren.

## Epic 5: System-Integration

**US-021** Als Nutzer möchte ich dass die App beim Systemstart automatisch startet, damit ich sie nicht manuell öffnen muss.

**US-022** Als Nutzer möchte ich die App im Systray als kleines Icon haben ohne Taskbar-Eintrag, damit sie nicht aufdringlich ist.

**US-023** Als Nutzer möchte ich automatische Updates erhalten, damit ich immer die aktuelle Version habe.

## Epic 6: ZeusX-Integration (Technisch)

**US-024** Als App möchte ich ZeusX-Buchungen per Browser-Automation ausführen, damit kein manueller Browser-Besuch nötig ist.

**US-025** Als App möchte ich stabile Selektoren nutzen (ID primär, Text als Fallback), damit DOM-Änderungen in ZeusX uns nicht brechen.

**US-026** Als App möchte ich Session-Extend im ZeusX-Fenster triggern, damit die Session nicht abläuft während die App läuft.

## Backlog / Nicht in v1

- **US-B01** Auto Check-In beim Arbeitsbeginn (config flag vorhanden, nicht implementiert)
- **US-B02** Auto Check-Out bei langer Inaktivität (config flag vorhanden, nicht implementiert)
- **US-B03** Kalender-Integration für Feiertage/Urlaubstage
- **US-B04** Flexible Arbeitszeitmodelle (Wochenkonten, Gleitzeit)
- **US-B05** Multi-Account ZeusX (verschiedene Mandate)
