# Swaply

Swaply est une application permettant de partager des playlists entre Spotify et Deezer de manière simple et rapide.

## Fonctionnalités

- Importez vos playlists Spotify.
- Convertissez-les pour les rendre compatibles avec Deezer.
- Partagez vos playlists avec vos amis, quelle que soit leur plateforme musicale.

## Prérequis

Avant de commencer, assurez-vous d'avoir les outils suivants installés sur votre machine :

1. **Node.js**  
   Téléchargez et installez Node.js depuis le site officiel :  
   [https://nodejs.org/](https://nodejs.org/)

2. **Rust**  
   Téléchargez et installez Rust via rustup :  
   [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

3. **Android Studio et SDK Android**  
   Téléchargez et installez Android Studio depuis le site officiel :  
   [https://developer.android.com/studio](https://developer.android.com/studio)  
   Assurez-vous d'avoir configuré le SDK Android et le NDK.

4. **Variables d'environnement pour Android**  
   Configurez les variables d'environnement suivantes :

   - `ANDROID_SDK` : chemin vers le répertoire du SDK Android.
   - `NDK_HOME` : chemin vers le répertoire du NDK Android.

   Par exemple, sous Linux ou macOS, ajoutez ceci à votre fichier `~/.bashrc` ou `~/.zshrc` :

   ```bash
   export ANDROID_SDK=/chemin/vers/android-sdk
   export NDK_HOME=/chemin/vers/android-ndk
   ```

   Sous Windows, configurez-les via les paramètres système.

## Installation

1. Clonez ce dépôt sur votre machine locale :

   ```bash
   git clone https://github.com/votre-utilisateur/swaptun.git
   ```

2. Accédez au répertoire du projet :

   ```bash
   cd swaptun
   ```

3. Installez les dépendances Node.js :

   ```bash
   npm install
   ```

4. Lancez l'application en mode développement pour Android :
   ```bash
   npm run tauri android dev
   ```

## Configuration pour CouchDB

Pour que l'application fonctionne avec CouchDB en local, assurez-vous que l'URL de votre machine locale est ajoutée dans le fichier `src-tauri/capabilities/default.json`.

```json
"permissions": [
    {
    "identifier": "http:default",
    "allow": [
        { "url": "addresse_ip_pc" },
    ]
    }
]
```

Vous devez remplacer `addresse_ip_pc` par l'adresse IP qui s'affiche dans cette ligne : ` Info Using xxx.xxx.xxx.xxx to access the development server.`

Cela permet à l'application d'accéder à CouchDB en local.

## Contribution

Les contributions sont les bienvenues ! N'hésitez pas à ouvrir une issue ou à soumettre une pull request.

## Licence

Ce projet est sous licence MIT. Consultez le fichier `LICENSE` pour plus d'informations.
