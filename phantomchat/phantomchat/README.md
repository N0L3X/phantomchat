# PhantomChat

**PhantomChat** ist ein experimenteller dezentraler Messenger, der sich an den
Prinzipien der maximalen Vertraulichkeit und Metadatenarmut orientiert.  Die
Anwendung kombiniert Ideen aus Moneros Stealth‑Adressierung mit der
Double‑Ratchet‑Verschlüsselung aus dem Signal‑Protokoll.  Nachrichten
werden relay‑agnostisch über Nostr‑kompatible Relays verbreitet.  Optional kann
eine Mixnet‑Schicht (z.&nbsp;B. Nym oder Tor) hinzugefügt werden, um den
Transport weiter zu verschleiern.  Der Sourcecode hier dient als
Minimalbeispiel und erste Arbeitsbasis für ein vollständiges MVP.

## Verzeichnisstruktur

```
phantomchat/
├── android/        # Platzhalter für die Android‑App (Kotlin, MVVM, Room)
├── cli/            # Kommandozeilen‑Client (hier Rust‑Code, unkompiliert)
├── core/           # Kernbibliothek (Schlüsselverwaltung, Ratchet, Envelope)
├── relays/         # Adapter für Nostr‑Relays und lokale In‑Memory‑Relays
├── infra/          # Docker‑Compose, CI‑Konfigurationen, SBOM
├── scripts/        # Hilfsskripte (z. B. Keygen, Demo‑Harness)
├── spec/           # Spezifikation, Protokollbeschreibungen und Diagramme
└── docs/           # Sicherheits‑ und Datenschutzdokumentation
```

Die Monorepo‑Struktur erleichtert den konsistenten Aufbau des Projekts,
ermöglicht modulare Tests und fördert reproduzierbare Builds.  Die
Kernkomponenten des MVP (Core‑Lib, CLI‑Client und lokale Relays) sind bereits
angelegt und teilweise implementiert.  Die Android‑App ist als Platzhalter
angelegt und soll später die Kernbibliothek via FFI einbinden.

## Schnellstart

Die hier bereitgestellte Referenzimplementierung nutzt Rust‑Code für die
Kernkomponenten und den CLI‑Client.  Eine vollständige Kompilierung ist in
dieser Umgebung nicht möglich, da keine Rust‑Toolchain vorhanden ist.  Die
Dateien dienen als Referenz für die Architektur und die geplante API.  Um die
Bibliothek und den CLI‑Client lokal zu kompilieren, installieren Sie eine
aktuelle Rust‑Toolchain (≥ 1.70) und führen Sie dann folgenden Befehl im
Projektverzeichnis aus:

```sh
cd phantomchat
cargo build --release
```

Für eine End‑zu‑End‑Demo wurde im Verzeichnis `scripts/` ein kleines
Testharness in Python hinterlegt.  Das Skript `demo_stub.py` simuliert die
Schlüsselerzeugung, verschlüsselt Nachrichten mit Platzhalterfunktionen und
transportiert sie über ein In‑Memory‑Relay.  Es ersetzt **nicht** die
Kryptographie der Kernbibliothek, kann aber den Nachrichtenfluss demonstrieren.

## Weiterführende Dokumentation

* `spec/SPEC.md` – detaillierte Beschreibung des Protokolls, des
  Nachrichtenformats und der Zustandsmaschinen.
* `docs/SECURITY.md` – Bedrohungsmodell, Sicherheitsannahmen und
  Abwehrmaßnahmen.
* `docs/PRIVACY.md` – Datenschutzprinzipien und Hinweise zum Schutz der
  Metadaten.

## Status

Die vorliegende Fassung stellt ein erstes Gerüst dar.  Viele Module sind noch
als Platzhalter ausgeführt und müssen mit Produktionscode hinterlegt werden.
Insbesondere fehlt eine echte Implementierung der Kryptographie, der
Double‑Ratchet‑Engine und der Relay‑Adapter.  Trotzdem bietet der Code eine
solide Grundlage für die weitere Entwicklung.