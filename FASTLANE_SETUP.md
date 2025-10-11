# Configuration Fastlane pour Swaptun

Ce document décrit comment configurer Fastlane pour automatiser le déploiement sur Google Play et l'App Store.

## 📁 Structure des dossiers

Vous devez créer deux dossiers à la racine du projet :

```
swaptun/
├── android/          # Configuration Fastlane pour Android
│   ├── Fastfile
│   └── Appfile
├── ios/              # Configuration Fastlane pour iOS
│   ├── Fastfile
│   └── Appfile
└── .github/
    └── workflows/    # Workflows GitHub Actions (déjà créés)
```

---

## 🤖 Configuration Android

### 1. Créer le dossier `android/` à la racine

```bash
mkdir -p android
cd android
```

### 2. Créer `android/Fastfile`

```ruby
default_platform(:android)

platform :android do
  desc "Deploy to Google Play Internal Testing"
  lane :closed_testing do
    upload_to_play_store(
      track: 'internal',
      aab: '../src-tauri/gen/android/app/build/outputs/bundle/universalRelease/app-universal-release.aab',
      json_key: '../google-play-key.json',
      skip_upload_apk: true,
      skip_upload_metadata: true,
      skip_upload_images: true,
      skip_upload_screenshots: true
    )
  end

  desc "Deploy to Google Play Production"
  lane :production do
    upload_to_play_store(
      track: 'production',
      aab: '../src-tauri/gen/android/app/build/outputs/bundle/universalRelease/app-universal-release.aab',
      json_key: '../google-play-key.json',
      skip_upload_apk: true,
      skip_upload_metadata: true,
      skip_upload_images: true,
      skip_upload_screenshots: true
    )
  end
end
```

### 3. Créer `android/Appfile`

```ruby
json_key_file("../google-play-key.json")
package_name("com.swaptun.app")
```

### 4. Configuration Google Play Console

#### a. Créer un compte de service

1. Allez sur [Google Cloud Console](https://console.cloud.google.com/)
2. Créez un nouveau projet ou sélectionnez celui lié à votre app
3. Activez l'API **Google Play Android Developer API**
4. Allez dans **IAM & Admin** > **Service Accounts**
5. Créez un compte de service avec le rôle **Service Account User**
6. Créez une clé JSON pour ce compte de service
7. Téléchargez le fichier JSON

#### b. Lier le compte de service à Google Play Console

1. Allez sur [Google Play Console](https://play.google.com/console/)
2. **Paramètres** > **Accès API**
3. Liez le projet Google Cloud
4. Donnez les permissions au compte de service :
   - **Releases** (Publier des versions)
   - **App access** (Accès à l'app)

#### c. Ajouter le secret GitHub

1. Encodez le fichier JSON en base64 :
   ```bash
   cat google-play-key.json | base64 -w 0
   ```

2. Allez dans **Settings** > **Secrets and variables** > **Actions** de votre repo GitHub

3. Ajoutez le secret :
   - Nom : `GOOGLE_PLAY_JSON_KEY`
   - Valeur : Le contenu base64 du fichier JSON

### 5. Configuration du Keystore Android

#### a. Encoder votre keystore en base64

```bash
cat keystore.jks | base64 -w 0
```

#### b. Ajouter les secrets GitHub

| Secret Name                  | Description                           | Exemple                    |
|------------------------------|---------------------------------------|----------------------------|
| `ANDROID_KEYSTORE_BASE64`    | Keystore encodé en base64             | `MIIJqwIBAzCC...`          |
| `ANDROID_KEYSTORE_PASSWORD`  | Mot de passe du keystore              | `myKeystorePassword`       |
| `ANDROID_KEY_ALIAS`          | Alias de la clé                       | `upload`                   |
| `ANDROID_KEY_PASSWORD`       | Mot de passe de la clé                | `myKeyPassword`            |

---

## 🍎 Configuration iOS

### 1. Créer le dossier `ios/` à la racine

```bash
mkdir -p ios
cd ios
```

### 2. Créer `ios/Fastfile`

```ruby
default_platform(:ios)

platform :ios do
  desc "Upload to TestFlight"
  lane :testflight do
    api_key = app_store_connect_api_key(
      key_id: ENV['APP_STORE_CONNECT_API_KEY_ID'],
      issuer_id: ENV['APP_STORE_CONNECT_API_ISSUER_ID'],
      key_content: ENV['APP_STORE_CONNECT_API_KEY'],
      is_key_content_base64: true
    )

    upload_to_testflight(
      api_key: api_key,
      ipa: '../src-tauri/gen/apple/build/arm64/swaptun.ipa',
      skip_waiting_for_build_processing: true,
      skip_submission: true
    )
  end

  desc "Upload to App Store"
  lane :production do
    api_key = app_store_connect_api_key(
      key_id: ENV['APP_STORE_CONNECT_API_KEY_ID'],
      issuer_id: ENV['APP_STORE_CONNECT_API_ISSUER_ID'],
      key_content: ENV['APP_STORE_CONNECT_API_KEY'],
      is_key_content_base64: true
    )

    upload_to_app_store(
      api_key: api_key,
      ipa: '../src-tauri/gen/apple/build/arm64/swaptun.ipa',
      skip_metadata: true,
      skip_screenshots: true,
      submit_for_review: false,
      automatic_release: false
    )
  end
end
```

### 3. Créer `ios/Appfile`

```ruby
app_identifier("com.swaptun.app")
apple_id(ENV['FASTLANE_APPLE_ID'])
team_id(ENV['FASTLANE_TEAM_ID'])
```

### 4. Configuration App Store Connect

#### a. Créer une clé API App Store Connect

1. Allez sur [App Store Connect](https://appstoreconnect.apple.com/)
2. **Users and Access** > **Keys** > **App Store Connect API**
3. Créez une nouvelle clé avec le rôle **Developer** ou **Admin**
4. Téléchargez le fichier `.p8`
5. Notez :
   - **Key ID** (ex: `AB12CD34EF`)
   - **Issuer ID** (ex: `12345678-1234-1234-1234-123456789012`)

#### b. Encoder la clé API en base64

```bash
cat AuthKey_AB12CD34EF.p8 | base64 -w 0
```

#### c. Certificats de signature

1. Allez dans **Xcode** > **Preferences** > **Accounts**
2. Ajoutez votre Apple ID
3. Téléchargez les certificats de distribution
4. Exportez le certificat en `.p12` :
   ```bash
   # Dans Keychain Access
   # Trouvez "Apple Distribution: Your Name (TEAM_ID)"
   # Export → Format: .p12
   ```

5. Encodez le certificat :
   ```bash
   cat Certificates.p12 | base64 -w 0
   ```

#### d. Provisioning Profile

1. Allez sur [Apple Developer Portal](https://developer.apple.com/account/)
2. **Certificates, Identifiers & Profiles** > **Profiles**
3. Créez un **App Store Distribution Profile**
4. Téléchargez le fichier `.mobileprovision`
5. Encodez-le :
   ```bash
   cat AppStore_com.swaptun.app.mobileprovision | base64 -w 0
   ```

#### e. Ajouter les secrets GitHub

| Secret Name                                  | Description                              | Exemple                       |
|----------------------------------------------|------------------------------------------|-------------------------------|
| `FASTLANE_APPLE_ID`                          | Votre Apple ID (email)                   | `dev@swaptun.com`             |
| `FASTLANE_APPLE_APP_SPECIFIC_PASSWORD`       | Mot de passe spécifique app              | `abcd-efgh-ijkl-mnop`         |
| `FASTLANE_TEAM_ID`                           | Team ID Apple Developer                  | `TJGS234P96`                  |
| `APP_STORE_CONNECT_API_KEY_ID`               | Key ID de l'API                          | `AB12CD34EF`                  |
| `APP_STORE_CONNECT_API_ISSUER_ID`            | Issuer ID de l'API                       | `12345678-1234-...`           |
| `APP_STORE_CONNECT_API_KEY`                  | Fichier .p8 encodé en base64             | `LS0tLS1CRUdJTi...`           |
| `IOS_CERTIFICATES_P12`                       | Certificat .p12 encodé en base64         | `MIIKJAIBAzCC...`             |
| `IOS_CERTIFICATES_PASSWORD`                  | Mot de passe du certificat .p12          | `certificatePassword`         |
| `IOS_PROVISIONING_PROFILE`                   | Provisioning profile encodé en base64    | `MIIMZwYJKoZI...`             |

---

## 🔐 Récapitulatif des secrets GitHub nécessaires

### Android (6 secrets)

1. `GOOGLE_PLAY_JSON_KEY` - Clé JSON du compte de service Google Play
2. `ANDROID_KEYSTORE_BASE64` - Keystore encodé en base64
3. `ANDROID_KEYSTORE_PASSWORD` - Mot de passe du keystore
4. `ANDROID_KEY_ALIAS` - Alias de la clé
5. `ANDROID_KEY_PASSWORD` - Mot de passe de la clé
6. `TAURI_SIGNING_PRIVATE_KEY` (optionnel pour Tauri)

### iOS (9 secrets)

1. `FASTLANE_APPLE_ID` - Apple ID (email)
2. `FASTLANE_APPLE_APP_SPECIFIC_PASSWORD` - Mot de passe spécifique app
3. `FASTLANE_TEAM_ID` - Team ID Apple Developer
4. `APP_STORE_CONNECT_API_KEY_ID` - Key ID de l'API App Store Connect
5. `APP_STORE_CONNECT_API_ISSUER_ID` - Issuer ID de l'API
6. `APP_STORE_CONNECT_API_KEY` - Fichier .p8 encodé en base64
7. `IOS_CERTIFICATES_P12` - Certificat .p12 encodé en base64
8. `IOS_CERTIFICATES_PASSWORD` - Mot de passe du certificat
9. `IOS_PROVISIONING_PROFILE` - Provisioning profile encodé en base64

---

## 🚀 Utilisation

### Pour déployer en test :

```bash
git push origin release-test
```

Cela déclenchera :
- ✅ Build Android + upload sur Google Play Internal Testing
- ✅ Build iOS + upload sur TestFlight

### Pour déployer en production :

```bash
git push origin release
```

Cela déclenchera :
- ✅ Build Android + upload sur Google Play Production
- ✅ Build iOS + upload sur App Store
- ✅ Création d'une GitHub Release avec les fichiers APK/AAB/IPA

---

## 📝 Notes importantes

### Chemins des fichiers build Tauri

- **Android APK** : `src-tauri/gen/android/app/build/outputs/apk/universal/release/`
- **Android AAB** : `src-tauri/gen/android/app/build/outputs/bundle/universalRelease/`
- **iOS IPA** : `src-tauri/gen/apple/build/arm64/`

### Commandes Tauri utilisées

```bash
# Android
cargo tauri android build --apk --aab

# iOS
cargo tauri ios build --export-method app-store
```

### Tester Fastlane localement

```bash
# Android
cd android
fastlane closed_testing

# iOS
cd ios
fastlane testflight
```

---

## 🔧 Dépannage

### Erreur "No such file or directory" pour l'AAB/IPA

Vérifiez que les chemins dans les Fastfiles correspondent exactement aux chemins générés par Tauri :

```bash
# Lister les fichiers générés
find src-tauri/gen -name "*.aab" -o -name "*.ipa"
```

### Erreur de signature Android

Assurez-vous que le keystore est correctement décodé et que les mots de passe sont corrects.

### Erreur iOS "No valid code signing identity"

Vérifiez que le certificat et le provisioning profile sont correctement importés dans le keychain temporaire.

---

## ✅ Checklist de configuration

- [ ] Dossiers `android/` et `ios/` créés
- [ ] Fichiers `Fastfile` et `Appfile` créés pour Android et iOS
- [ ] Compte de service Google Play créé et clé JSON téléchargée
- [ ] API App Store Connect créée et clé .p8 téléchargée
- [ ] Certificats iOS exportés en .p12
- [ ] Provisioning profile téléchargé
- [ ] Tous les secrets GitHub ajoutés
- [ ] Keystore Android encodé et ajouté aux secrets
- [ ] Test local des lanes Fastlane
- [ ] Push sur `release-test` pour tester les workflows

---

**Bon déploiement ! 🚀**
