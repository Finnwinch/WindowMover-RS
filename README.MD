# 🪟 WindowMover-RS

Un utilitaire ultra-léger (1mb) développé en **Rust** pour Windows qui permet de déplacer la fenêtre active (focus) à l'aide de raccourcis clavier.

## 🚀 Pourquoi WindowMover ?
- **Performance** : Écrit en Rust natif, consommation CPU/RAM quasi nulle.
- **Discrétion** : S'exécute en arrière-plan sans fenêtre de console (`windows_subsystem`).
- **Simplicité** : Pas d'interface complexe, juste de l'efficacité.

---

## ⌨️ Utilisation

Maintenez la touche **Shift Droit (RShift)** enfoncée, puis utilisez les touches suivantes :

| Touche | Action |
| :--- | :--- |
| **W** | Déplacer vers le **Haut** |
| **S** | Déplacer vers le **Bas** |
| **A** | Déplacer vers la **Gauche** |
| **D** | Déplacer vers la **Droite** |

> **Note** : Le déplacement se fait par paliers de 10 pixels. Vous pouvez ajuster la variable `step` dans le code source si nécessaire.

---

## 🛠️ Compilation et Installation

### Prérequis
- [Rust & Cargo](https://rustup.rs/) (dernière version stable recommandée).
- Windows 10 ou 11.
