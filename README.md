# phantomchat
Dezentraler Messenger

# PhantomChat

**PhantomChat** ist ein experimenteller, hochsicherer, dezentraler Messenger,
der maximale Vertraulichkeit und minimale Metadaten-Leaks zum Ziel hat.  
Er kombiniert drei Kernideen:

* **Stealth-Adressierung (Monero-Style):**  
  Empfänger scannen mit einem `view_key` nach geheimen Tags.  
  Nur sie erkennen „diese Nachricht ist für mich“.  
  Ein `spend_key` wird zum Entschlüsseln verwendet.

* **Signal-Double-Ratchet-Verschlüsselung:**  
  Jede Nachricht bekommt frische Schlüssel (Forward Secrecy & Post-Compromise Security).

* **Relay-Agnostischer Transport (Nostr-Style):**  
  Nachrichten werden über mehrere Relays parallel verteilt (mind. 3),  
  inklusive Health-Scoring, Multipath und Deduplizierung.  
  Optional kann ein Mixnet-Layer (Tor/Nym) aktiviert werden.

---

## Status

⚠️ **MVP-Prototyp**  
Dies ist ein Proof-of-Concept und **kein fertiges Produkt**.  
Der Rust-Code enthält Platzhalter für die eigentliche Kryptografie und
Relay-Anbindung.  
Eine Python-Demo (`scripts/demo_stub.py`) zeigt die Abläufe mit vereinfachter
Kryptographie.

---

## Projektstruktur

```

phantomchat/
├─ android/        # (geplant) Android-App mit Kotlin MVVM & Room (SQLCipher)
├─ cli/            # Rust-CLI-Client (Keygen, Pairing, Senden/Empfangen)
├─ core/           # Rust-Kernbibliothek (Schlüssel, Ratchet, Envelope, PoW)
├─ relays/         # Relay-Adapter (In-Memory & Nostr-Placeholder)
├─ spec/           # Protokollspezifikation & Diagramme (SPEC.md)
├─ docs/           # SECURITY.md, PRIVACY.md, weitere Dokumente
├─ infra/          # docker-compose.yml für lokale Relays
└─ scripts/        # demo\_stub.py: End-to-End-Demo (Stub-Krypto)

````

---

## Features (MVP)

* **Pairing via QR:** Austausch von `view_pub`, `spend_pub` & human-readable Fingerprint.
* **Ende-zu-Ende-Textnachrichten:** Double Ratchet mit X25519 → HKDF → XChaCha20-Poly1305.
* **Stealth-Adressierung:** Empfänger erkennt Nachrichten nur durch geheime HMAC-Tags.
* **Relays:** Nostr-kompatibel, Health-Scoring, Rotation, Backoff, Deduplizierung.
* **Spam-Schutz:** Hashcash (z. B. 20 führende Nullbits) über Event-Header.
* **Lokale Verschlüsselung:**  
  - Android: Keystore + SQLCipher/Room  
  - CLI: Argon2id + sealed boxes (libsodium, geplant)
* **Optionale Hard-Mode-Schalter:** Mixnet, Cover-Traffic, Post-Quanten-Hybrid (Kyber).

---

## Demo starten

Voraussetzung: Python 3

```bash
cd scripts
python3 demo_stub.py
````

Erwartete Ausgabe:
`Bob hat Nachricht empfangen: (msg_id=...)`

Dies demonstriert Schlüsselerzeugung, Stealth-Tag-Berechnung,
Proof-of-Work und Nachrichtenzustellung **ohne echte Krypto**.

---

## Entwicklung & Build (Core/CLI)

1. Rust installieren ([https://rustup.rs](https://rustup.rs))
2. In den Ordner wechseln, z. B. `core`:

   ```bash
   cd core
   cargo build
   cargo test
   ```

   Gleiches für `cli` und `relays`.

> Die Kryptographie-Module enthalten aktuell Platzhalter.
> Für einen produktiven Build müssen reale Krypto-Crates (x25519-dalek,
> chacha20poly1305, hkdf etc.) ergänzt werden.

---

## Infrastruktur / lokale Relays

Docker Compose kann mehrere Relays für Tests starten:

```bash
cd infra
docker compose up -d
```

Standardports: `7000`, `7001`, `7002`
Diese Relays können in der CLI oder späteren Android-App konfiguriert werden.

---

## Sicherheit & Datenschutz

* **E2E-Verschlüsselung & Forward Secrecy** (Signal-Style Double Ratchet)
* **Keine zentralen Identitäten**, keine Telefonnummern, kein Cloud-Backup
* **Stealth-Tags** verhindern Metadaten-Leaks
* **Optionale Mixnets** für stärkere Anonymität

Details siehe:

* [`docs/SECURITY.md`](docs/SECURITY.md)
* [`docs/PRIVACY.md`](docs/PRIVACY.md)

---

## Lizenz

Wähle eine Lizenz nach deinem Bedarf:

* **AGPL-3.0** (starkes Copyleft für offene Weitergabe)
* **MPL-2.0** (leichtes Copyleft)
* **Apache-2.0** (permissiv)

---

## Roadmap

* [ ] Echte Krypto mit `x25519-dalek`, `chacha20poly1305`, `hkdf`, `hmac`
* [ ] Vollständiger Nostr-Adapter mit Multipath, Dedupe, ACK/TTL
* [ ] Android-App (MVVM, Jetpack Security, QR-Pairing)
* [ ] Post-Quantum-Hybrid (Kyber/X25519) als Option
* [ ] Gruppen- und Medien-Support (später)

---

**Hinweis:**
Dieses Projekt befindet sich in der Forschung/Experimentierphase und ist
**nicht** für produktive oder sicherheitskritische Kommunikation geeignet.

```

---

Möchtest du, dass ich den Text noch um z. B. **Build-Status-Badges** (GitHub Actions), **Lizenz-Hinweise** oder ein **kurzes Demo-GIF/Screenshot-Konzept** ergänze, bevor wir ihn so bei GitHub einchecken?
```
