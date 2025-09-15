//! Double‑Ratchet‑Engine
//!
//! Diese Umsetzung ist stark vereinfacht und dient lediglich als
//! Anschauungsobjekt.  Der vollständige Double‑Ratchet umfaßt viele
//! Details wie KDF‑Ketten für Root, Senden und Empfangen, das
//! Speichern übersprungener Nachrichtenschlüssel, Headerverschlüsselung
//! usw.  Hier implementieren wir lediglich einen rudimentären
//! Nachrichten‑Zähler und erzeugen pro Nachricht einen neuen
//! Message‑Key.

use crate::keys::{ViewKey, SpendKey};
use crate::util::sha256;
use rand_core::OsRng;
use x25519_dalek::{PublicKey, StaticSecret};

/// Fehler, der bei der Ratchet‑Verarbeitung auftreten kann.
#[derive(Debug, thiserror::Error)]
pub enum RatchetError {
    #[error("Entschlüsselung fehlgeschlagen")]
    DecryptionFailed,
}

/// Zustand einer Double‑Ratchet‑Session.
#[derive(Debug, Clone)]
pub struct RatchetState {
    /// Root‑Key aus der letzten DH‑Operation.
    root_key: [u8; 32],
    /// Aktueller Sende‑Chain‑Key.
    send_chain: [u8; 32],
    /// Aktueller Empfangs‑Chain‑Key.
    recv_chain: [u8; 32],
    /// Aktuelles Ratchet‑Keypair (privat).
    ratchet_secret: StaticSecret,
    /// Aktueller Ratchet‑Public‑Key des Peers.
    peer_ratchet_public: PublicKey,
    /// Nachrichtenzähler für Sendekette.
    send_count: u32,
    /// Nachrichtenzähler für Empfangskette.
    recv_count: u32,
}

impl RatchetState {
    /// Initialisiert einen neuen Ratchet‑Zustand.  Dazu werden ein
    /// gemeinsamer Root‑Key und das anfängliche Ratchet‑Keypair
    /// benötigt.
    pub fn new(root_key: [u8; 32], peer_ratchet_public: PublicKey) -> Self {
        // Generiere eigenes Ratchet‑Keypair
        let ratchet_secret = StaticSecret::new(&mut OsRng);
        // Ableitung der Anfangs‑Chain‑Keys: in der echten
        // Implementierung erfolgt dies per HKDF aus dem Root‑Key und dem
        // DH‑Output; hier verwenden wir einfach Hashes zur Demonstration.
        let send_chain = sha256(&[0u8]);
        let recv_chain = sha256(&[1u8]);
        Self {
            root_key,
            send_chain: send_chain.try_into().expect("32 bytes"),
            recv_chain: recv_chain.try_into().expect("32 bytes"),
            ratchet_secret,
            peer_ratchet_public,
            send_count: 0,
            recv_count: 0,
        }
    }
    /// Erzeugt einen neuen Nachrichten‑Schlüssel aus der Sende‑Kette und
    /// erhöht den Zähler.  In der echten Implementierung wird dabei ein
    /// KDF (z.&nbsp;B. HMAC/SHA‑256) genutzt; hier wird zur Vereinfachung
    /// SHA‑256 über die Kette und den Zähler gebildet.
    fn next_send_key(&mut self) -> [u8; 32] {
        self.send_count += 1;
        let mut data = Vec::new();
        data.extend_from_slice(&self.send_chain);
        data.extend_from_slice(&self.send_count.to_le_bytes());
        let digest = sha256(&data);
        digest.try_into().expect("32 bytes")
    }
    /// Entschlüsselt eine Nachricht anhand der Empfangs‑Kette.  Auch
    /// hier wird nur eine Hashfunktion genutzt, um den Message‑Key
    /// abzuleiten.  Bei jedem Aufruf erhöht sich der Empfangs‑Zähler.
    fn next_recv_key(&mut self) -> [u8; 32] {
        self.recv_count += 1;
        let mut data = Vec::new();
        data.extend_from_slice(&self.recv_chain);
        data.extend_from_slice(&self.recv_count.to_le_bytes());
        let digest = sha256(&data);
        digest.try_into().expect("32 bytes")
    }
    /// Verschlüsselt eine Nachricht.  Diese Funktion gibt den
    /// Message‑Key und einen Platzhalter‑Header zurück.  In der
    /// vollständigen Umsetzung würde hier der Ratchet‑Header mit
    /// Ratchet‑Public‑Key und Positionsindizes serialisiert.
    pub fn encrypt(&mut self, plaintext: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let msg_key = self.next_send_key();
        // TODO: echte AEAD‑Verschlüsselung durch XChaCha20‑Poly1305
        // Hier wird zur Demo der Klartext einfach zurückgegeben.
        let ciphertext = plaintext.to_vec();
        // Header kann den aktuellen Ratchet‑Public‑Key enthalten
        let header = self.ratchet_secret.public_key().as_bytes().to_vec();
        (ciphertext, header)
    }
    /// Entschlüsselt eine Nachricht.  Für die Demonstration wird der
    /// Klartext direkt zurückgegeben.  In einer echten Implementierung
    /// würde der Header geprüft und ggf. ein DH‑Ratchet durchgeführt.
    pub fn decrypt(&mut self, _header: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, RatchetError> {
        let _msg_key = self.next_recv_key();
        // TODO: echte AEAD‑Entschlüsselung mit msg_key
        Ok(ciphertext.to_vec())
    }
}
