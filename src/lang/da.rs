lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Status"),
        ("Your Desktop", "Dit skrivebord"),
        ("desk_tip", "Du kan få adgang til dit skrivebord med dette ID og adgangskode."),
        ("outgoing_only_desk_tip", ""),
        ("Password", "Adgangskode"),
        ("Ready", "Klar"),
        ("Established", "Etableret"),
        ("connecting_status", "Opretter forbindelse til RustDesk-netværket..."),
        ("Enable service", "Tænd forbindelsesserveren"),
        ("Start service", "Start forbindelsesserveren"),
        ("Service is running", "Tjenesten kører"),
        ("Service is not running", "Den tilknyttede tjeneste kører ikke"),
        ("not_ready_status", "Ikke klar. Tjek venligst din forbindelse"),
        ("Control Remote Desktop", "Styr fjernskrivebord"),
        ("Transfer file", "Overfør fil"),
        ("Connect", "Forbind"),
        ("Recent sessions", "Seneste sessioner"),
        ("Address book", "Adressebog"),
        ("Confirmation", "Bekræftelse"),
        ("TCP tunneling", "TCP tunneling"),
        ("Remove", "Fjern"),
        ("Refresh random password", "Opdater tilfældig adgangskode"),
        ("Set your own password", "Indstil din egen adgangskode"),
        ("Enable keyboard/mouse", "Tænd for tastatur/mus"),
        ("Enable clipboard", "Tænd for udklipsholderen"),
        ("Enable file transfer", "Aktivér filoverførsel"),
        ("Enable TCP tunneling", "Slå TCP-tunneling til"),
        ("IP Whitelisting", "IP whitelisting"),
        ("ID/Relay Server", "ID/forbindelsesserver"),
        ("Import server config", "Importér serverkonfiguration"),
        ("Export Server Config", "Eksportér serverkonfiguration"),
        ("Import server configuration successfully", "Importering af serverkonfigurationen lykkedes"),
        ("Export server configuration successfully", "Eksportering af serverkonfigurationen lykkedes"),
        ("Invalid server configuration", "Ugyldig serverkonfiguration"),
        ("Clipboard is empty", "Udklipsholderen er tom"),
        ("Stop service", "Sluk for forbindelsesserveren"),
        ("Change ID", "Ændr ID"),
        ("Your new ID", "Dit nye ID"),
        ("length %min% to %max%", ""),
        ("starts with a letter", ""),
        ("allowed characters", ""),
        ("id_change_tip", "Kun tegnene a-z, A-Z, 0-9 og _ (understregning) er tilladt. Det første bogstav skal være a-z, A-Z. Længde mellem 6 og 16."),
        ("Website", "Hjemmeside"),
        ("About", "Om"),
        ("Slogan_tip", ""),
        ("Privacy Statement", ""),
        ("Mute", "Sluk for mikrofonen"),
        ("Build Date", ""),
        ("Version", ""),
        ("Home", ""),
        ("Audio Input", "Lydinput"),
        ("Enhancements", "Forbedringer"),
        ("Hardware Codec", "Hardware-codec"),
        ("Adaptive bitrate", "Adaptiv bitrate"),
        ("ID Server", "ID Server"),
        ("Relay Server", "Relay Server"),
        ("API Server", "API Server"),
        ("invalid_http", "Skal begynde med http:// eller https://"),
        ("Invalid IP", "Ugyldig IP-adresse"),
        ("Invalid format", "Ugyldigt format"),
        ("server_not_support", "Endnu ikke understøttet af serveren"),
        ("Not available", "Ikke tilgængelig"),
        ("Too frequent", "For ofte"),
        ("Cancel", "Annullér"),
        ("Skip", "Spring over"),
        ("Close", "Luk"),
        ("Retry", "Prøv igen"),
        ("OK", "OK"),
        ("Password Required", "Adgangskode påkrævet"),
        ("Please enter your password", "Indtast venligst dit kodeord"),
        ("Remember password", "Husk kodeord"),
        ("Wrong Password", "Forkert kodeord"),
        ("Do you want to enter again?", "Vil du forbinde igen?"),
        ("Connection Error", "Forbindelsesfejl"),
        ("Error", "Fejl"),
        ("Reset by the peer", "Nulstillet ved modparten"),
        ("Connecting...", "Opretter forbindelse..."),
        ("Connection in progress. Please wait.", "Forbindelsen er etableret. Vent venligst."),
        ("Please try 1 minute later", "Prøv igen om et minut"),
        ("Login Error", "Login fejl"),
        ("Successful", "Vellykket"),
        ("Connected, waiting for image...", "Tilsluttet, venter på billede..."),
        ("Name", "Navn"),
        ("Type", "Type"),
        ("Modified", "Ændret"),
        ("Size", "Størrelse"),
        ("Show Hidden Files", "Vis skjulte filer"),
        ("Receive", "Modtag"),
        ("Send", "Send"),
        ("Refresh File", "Genopfrisk fil"),
        ("Local", "Lokalt"),
        ("Remote", "Remote"),
        ("Remote Computer", "Fjerncomputer"),
        ("Local Computer", "Lokal computer"),
        ("Confirm Delete", "Bekræft sletning"),
        ("Delete", "Slet"),
        ("Properties", "Egenskaber"),
        ("Multi Select", "Flere valg"),
        ("Select All", "Vælg alt"),
        ("Unselect All", "Fravælg alt"),
        ("Empty Directory", "Tomt bibliotek"),
        ("Not an empty directory", "Intet tomt bibliotek"),
        ("Are you sure you want to delete this file?", "Er du sikker på, at du vil slette denne fil?"),
        ("Are you sure you want to delete this empty directory?", "Er du sikker på, at du vil slette dette tomme bibliotek?"),
        ("Are you sure you want to delete the file of this directory?", "Er du sikker på, at du vil slette filen til dette bibliotek?"),
        ("Do this for all conflicts", "Gør dette for alle konflikter"),
        ("This is irreversible!", "Dette kan ikke gendannes!"),
        ("Deleting", "Sletter"),
        ("files", "Filer"),
        ("Waiting", "Venter"),
        ("Finished", "Færdig"),
        ("Speed", "Hastighed"),
        ("Custom Image Quality", "Brugerdefineret billedkvalitet"),
        ("Privacy mode", "Privatlivstilstand"),
        ("Block user input", "Bloker brugerinput"),
        ("Unblock user input", "Fjern blokering af brugerinput"),
        ("Adjust Window", "Juster vinduet"),
        ("Original", "Original"),
        ("Shrink", "Krymp"),
        ("Stretch", "Stræk ud"),
        ("Scrollbar", "Rullebar"),
        ("ScrollAuto", "Auto-rul"),
        ("Good image quality", "God billedkvalitet"),
        ("Balanced", "Afbalanceret"),
        ("Optimize reaction time", "Optimeret responstid"),
        ("Custom", "Tilpasset"),
        ("Show remote cursor", "Vis fjernbetjeningskontrolleret markør"),
        ("Show quality monitor", "Vis billedkvalitet"),
        ("Disable clipboard", "Deaktiver udklipsholder"),
        ("Lock after session end", "Lås efter afslutningen af fjernstyring"),
        ("Insert", "Indsæt"),
        ("Insert Lock", "Indsæt lås"),
        ("Refresh", "Genopfrisk"),
        ("ID does not exist", "ID findes ikke"),
        ("Failed to connect to rendezvous server", "Forbindelse til forbindelsesserveren mislykkedes"),
        ("Please try later", "Prøv igen senere"),
        ("Remote desktop is offline", "Fjernskrivebord er offline"),
        ("Key mismatch", "Nøgle uoverensstemmelse"),
        ("Timeout", "Timeout"),
        ("Failed to connect to relay server", "Forbindelse til relæ-serveren mislykkedes"),
        ("Failed to connect via rendezvous server", "Forbindelse via Rendezvous-server mislykkedes"),
        ("Failed to connect via relay server", "Forbindelse via relæ-serveren mislykkedes"),
        ("Failed to make direct connection to remote desktop", "Direkte forbindelse til fjernskrivebord kunne ikke etableres"),
        ("Set Password", "Indstil adgangskode"),
        ("OS Password", "Operativsystemadgangskode"),
        ("install_tip", "På grund af UAC kan RustDesk ikke fungere korrekt i nogle tilfælde på fjernskrivebordet. For at undgå UAC skal du klikke på knappen nedenfor for at installere RustDesk på systemet"),
        ("Click to upgrade", "Klik for at opgradere"),
        ("Click to download", "Klik for at downloade"),
        ("Click to update", "Klik for at opdatere"),
        ("Configure", "Konfigurer"),
        ("config_acc", "For at kontrollere dit skrivebord på afstand skal du give RustDesk \"Access \" Rettigheder."),
        ("config_screen", "For at kunne få adgang til dit skrivebord langtfra, skal du give RustDesk \"skærmstøtte \" tilladelser."),
        ("Installing ...", "Installerer ..."),
        ("Install", "installere"),
        ("Installation", "Installation"),
        ("Installation Path", "Installationsti"),
        ("Create start menu shortcuts", "Opret start menu genveje"),
        ("Create desktop icon", "Opret skrivebords-genvej"),
        ("agreement_tip", "Hvis du starter installationen, skal du acceptere licensaftalen"),
        ("Accept and Install", "Accepter og installer"),
        ("End-user license agreement", "Licensaftale for slutbrugere"),
        ("Generating ...", "Genererer kode ..."),
        ("Your installation is lower version.", "Din installation er en ældre version."),
        ("not_close_tcp_tip", "Luk ikke dette vindue, mens du bruger tunnelen."),
        ("Listening ...", "Lytter ..."),
        ("Remote Host", "Fjern-Host"),
        ("Remote Port", "Fjern-Port"),
        ("Action", "Handling"),
        ("Add", "Tilføj"),
        ("Local Port", "Lokal Port"),
        ("Local Address", "Lokal adresse"),
        ("Change Local Port", "Skift lokal port"),
        ("setup_server_tip", "For en hurtigere forbindelse skal du indstille din egen forbindelsesserver"),
        ("Too short, at least 6 characters.", "For kort, brug mindst 6 tegn."),
        ("The confirmation is not identical.", "Bekræftelsen er ikke identisk."),
        ("Permissions", "Tilladelser"),
        ("Accept", "Accepter"),
        ("Dismiss", "Afvis"),
        ("Disconnect", "Frakobl"),
        ("Enable file copy and paste", "Tillad kopiering og indsæt af filer"),
        ("Connected", "Forbundet"),
        ("Direct and encrypted connection", "Direkte og krypteret forbindelse"),
        ("Relayed and encrypted connection", "Viderestillet og krypteret forbindelse"),
        ("Direct and unencrypted connection", "Direkte og ukrypteret forbindelse"),
        ("Relayed and unencrypted connection", "Viderestillet og ukrypteret forbindelse"),
        ("Enter Remote ID", "Indtast Remote-ID"),
        ("Enter your password", "Skriv dit kodeord"),
        ("Logging in...", "Logger ind..."),
        ("Enable RDP session sharing", "Aktivér RDP sessiongodkendelse"),
        ("Auto Login", "Automatisk login (kun gyldigt hvis du har konfigureret \"Lås efter afslutningen af sessionen\")"),
        ("Enable direct IP access", "Aktivér direkte IP-adgang"),
        ("Rename", "Omdøb"),
        ("Space", "Plads"),
        ("Create desktop shortcut", "Opret skrivebords-genvej"),
        ("Change Path", "Skift stien"),
        ("Create Folder", "Opret mappe"),
        ("Please enter the folder name", "Indtast venligst mappens navn"),
        ("Fix it", "Kør reparation"),
        ("Warning", "Advarsel"),
        ("Login screen using Wayland is not supported", "Login skærm med Wayland understøttes ikke"),
        ("Reboot required", "Genstart krævet"),
        ("Unsupported display server", "Ikke-understøttet displayserver"),
        ("x11 expected", "X11 Forventet"),
        ("Port", "Port"),
        ("Settings", "Indstillinger"),
        ("Username", " Brugernavn"),
        ("Invalid port", "Ugyldig port"),
        ("Closed manually by the peer", "Manuelt lukket af peer"),
        ("Enable remote configuration modification", "Tillad fjernkonfigurering"),
        ("Run without install", "Kør uden installation"),
        ("Connect via relay", "Forbind via viderestilling"),
        ("Always connect via relay", "Forbindelse via viderestillings-server"),
        ("whitelist_tip", "Kun IP'er på whitelisten kan få adgang til mig"),
        ("Login", "Login"),
        ("Verify", "Verificér"),
        ("Remember me", "Husk mig"),
        ("Trust this device", "Husk denne enhed"),
        ("Verification code", "Verifikationskode"),
        ("verification_tip", ""),
        ("Logout", "Logger af"),
        ("Tags", "Nøgleord"),
        ("Search ID", "Søg efter ID"),
        ("whitelist_sep", "Adskilt af komma, semikolon, mellemrum eller linjebrud"),
        ("Add ID", "Tilføj ID"),
        ("Add Tag", "Tilføj nøgleord"),
        ("Unselect all tags", "Fravælg alle nøgleord"),
        ("Network error", "Netværksfejl"),
        ("Username missed", "Glemt brugernavn"),
        ("Password missed", "Glemt kodeord"),
        ("Wrong credentials", "Forkerte registreringsdata"),
        ("The verification code is incorrect or has expired", ""),
        ("Edit Tag", "Rediger nøgleord"),
        ("Forget Password", "Glem adgangskoden"),
        ("Favorites", "Favoritter"),
        ("Add to Favorites", "Tilføj til favoritter"),
        ("Remove from Favorites", "Fjern favoritter"),
        ("Empty", "Tom"),
        ("Invalid folder name", "Ugyldigt mappenavn"),
        ("Socks5 Proxy", "Socks5 Proxy"),
        ("Hostname", "Computernavn"),
        ("Discovered", "Fundet"),
        ("install_daemon_tip", "For at starte efter PC'en er startet op, skal du installere systemtjenesten"),
        ("Remote ID", "Fjern-ID"),
        ("Paste", "Indsæt"),
        ("Paste here?", "Indsæt her?"),
        ("Are you sure to close the connection?", "Er du sikker på at du vil afslutte forbindelsen?"),
        ("Download new version", "Download ny version"),
        ("Touch mode", "Touch-tilstand"),
        ("Mouse mode", "Musse-tilstand"),
        ("One-Finger Tap", "En-finger-tryk"),
        ("Left Mouse", "Venstre mus"),
        ("One-Long Tap", "Tryk og hold med en finger"),
        ("Two-Finger Tap", "Tryk med to fingre"),
        ("Right Mouse", "Højre mus"),
        ("One-Finger Move", "En-finger bevægelse"),
        ("Double Tap & Move", "Dobbeltklik og flyt"),
        ("Mouse Drag", "Træk med musen"),
        ("Three-Finger vertically", "Tre fingre lodret"),
        ("Mouse Wheel", "Mussehjul"),
        ("Two-Finger Move", "To-fingre bevægelse"),
        ("Canvas Move", "Flyt lærred"),
        ("Pinch to Zoom", "Knib for at zoome ind"),
        ("Canvas Zoom", "Lærred zoom"),
        ("Reset canvas", "Nulstil lærred"),
        ("No permission of file transfer", "Ingen tilladelse til at overføre filen"),
        ("Note", "Note"),
        ("Connection", "Forbindelse"),
        ("Share Screen", "Del skærmen"),
        ("Chat", "Chat"),
        ("Total", "Total"),
        ("items", "artikel"),
        ("Selected", "Valgte"),
        ("Screen Capture", "Skærmoptagelse"),
        ("Input Control", "Inputkontrol"),
        ("Audio Capture", "Lydoptagelse"),
        ("File Connection", "Filforbindelse"),
        ("Screen Connection", "Færdiggørelse"),
        ("Do you accept?", "Accepterer du?"),
        ("Open System Setting", "Åbn systemindstillingen"),
        ("How to get Android input permission?", "Hvordan får jeg en Android-input tilladelse?"),
        ("android_input_permission_tip1", "For at en ekstern enhed kan kontrollere din Android-enhed via mus eller berøring, skal du give RustDesk mulighed for at bruge tjenesten \"tilgængelighed \"."),
        ("android_input_permission_tip2", "Gå til den næste systemindstillingsside, søg og indtast [installerede tjenester], tænd for [RustDesk Input] tjenesten."),
        ("android_new_connection_tip", "En ny anmodning blev modtaget, der gerne vil kontrollere din nuværende enhed."),
        ("android_service_will_start_tip", "Ved at tænde for skærmoptagelsen startes tjenesten automatisk, så andre enheder kan anmode om en forbindelse fra denne enhed."),
        ("android_stop_service_tip", "Ved at lukke tjenesten lukkes alle fremstillede forbindelser automatisk."),
        ("android_version_audio_tip", "Den aktuelle Android-version understøtter ikke lydoptagelse. Android 10 eller højere er påkrævet."),
        ("android_start_service_tip", ""),
        ("android_permission_may_not_change_tip", ""),
        ("Account", "Konto"),
        ("Overwrite", "Overskriv"),
        ("This file exists, skip or overwrite this file?", "Denne fil findes allerede, vil du springe over eller overskrive denne fil?"),
        ("Quit", "Afslut"),
        ("Help", "Hjælp"),
        ("Failed", "Mislykkedet"),
        ("Succeeded", "Vellykket"),
        ("Someone turns on privacy mode, exit", "Nogen aktiverede privatlivstilstand, afslut"),
        ("Unsupported", "Ikke understøttet"),
        ("Peer denied", "Modpart nægtet"),
        ("Please install plugins", "Installer venligst plugins"),
        ("Peer exit", "Modpart-Afslut"),
        ("Failed to turn off", "Mislykkedes i at lukke ned"),
        ("Turned off", "Slukket"),
        ("Language", "Sprog"),
        ("Keep RustDesk background service", "Behold RustDesk baggrundstjeneste"),
        ("Ignore Battery Optimizations", "Ignorér betteri optimeringer"),
        ("android_open_battery_optimizations_tip", ""),
        ("Start on boot", "Start under opstart"),
        ("Start the screen sharing service on boot, requires special permissions", "Start skærmdelingstjenesten under opstart, kræver specielle rettigheder"),
        ("Connection not allowed", "Forbindelse ikke tilladt"),
        ("Legacy mode", "Bagudkompatibilitetstilstand"),
        ("Map mode", "Kortmodus"),
        ("Translate mode", "Oversættelsesmodus"),
        ("Use permanent password", "Brug permanent adgangskode"),
        ("Use both passwords", "Brug begge adgangskoder"),
        ("Set permanent password", "Sæt permanent adgangskode"),
        ("Enable remote restart", "Aktivér fjerngenstart"),
        ("Restart remote device", "Genstart fjernenhed"),
        ("Are you sure you want to restart", "Er du sikker på at du vil genstarte"),
        ("Restarting remote device", "Genstarter fjernenhed"),
        ("remote_restarting_tip", "Enheden genstarter - Lukker denne besked ned, og tilslutter igen om et øjeblik"),
        ("Copied", "Kopieret"),
        ("Exit Fullscreen", "Afslut fuldskærm"),
        ("Fullscreen", "Fuld skærm"),
        ("Mobile Actions", "Mobile handlinger"),
        ("Select Monitor", "Vælg skærm"),
        ("Control Actions", "Kontrolhandlinger"),
        ("Display Settings", "Skærmindstillinger"),
        ("Ratio", "Forhold"),
        ("Image Quality", "Billedkvalitet"),
        ("Scroll Style", "Rullestil"),
        ("Show Toolbar", ""),
        ("Hide Toolbar", ""),
        ("Direct Connection", "Direkte forbindelse"),
        ("Relay Connection", "Viderestillingsforbindelse"),
        ("Secure Connection", "Sikker forbindelse"),
        ("Insecure Connection", "Usikker forbindelse"),
        ("Scale original", "Original skalering"),
        ("Scale adaptive", "Adaptiv skalering"),
        ("General", "Generelt"),
        ("Security", "Sikkerhed"),
        ("Theme", "Thema"),
        ("Dark Theme", "Mørk Tema"),
        ("Light Theme", "Lys Tema"),
        ("Dark", "Mørk"),
        ("Light", "Lys"),
        ("Follow System", "Følg System"),
        ("Enable hardware codec", "Aktivér hardware-codec"),
        ("Unlock Security Settings", "Lås op for sikkerhedsindstillinger"),
        ("Enable audio", "Aktivér Lyd"),
        ("Unlock Network Settings", "Lås op for Netværksindstillinger"),
        ("Server", "Server"),
        ("Direct IP Access", "Direkte IP Adgang"),
        ("Proxy", "Proxy"),
        ("Apply", "Anvend"),
        ("Disconnect all devices?", "Afbryd alle enheder?"),
        ("Clear", "Nulstil"),
        ("Audio Input Device", "Lydindgangsenhed"),
        ("Use IP Whitelisting", "Brug IP Whitelisting"),
        ("Network", "Netværk"),
        ("Pin Toolbar", ""),
        ("Unpin Toolbar", ""),
        ("Recording", "Optager"),
        ("Directory", "Mappe"),
        ("Automatically record incoming sessions", "Optag automatisk indgående sessioner"),
        ("Change", "Ændr"),
        ("Start session recording", "Start sessionsoptagelse"),
        ("Stop session recording", "Stop sessionsoptagelse"),
        ("Enable recording session", "Aktivér optagelsessession"),
        ("Enable LAN discovery", "Aktivér LAN Discovery"),
        ("Deny LAN discovery", "Afvis LAN Discovery"),
        ("Write a message", "Skriv en besked"),
        ("Prompt", "Prompt"),
        ("Please wait for confirmation of UAC...", "Vent venligst på UAC-bekræftelse..."),
        ("elevated_foreground_window_tip", ""),
        ("Disconnected", "Afbrudt"),
        ("Other", "Andre"),
        ("Confirm before closing multiple tabs", "Bekræft før du lukker flere faner"),
        ("Keyboard Settings", "Tastaturindstillinger"),
        ("Full Access", "Fuld adgang"),
        ("Screen Share", "Skærmdeling"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland kræver Ubuntu version 21.04 eller nyere."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland kræver en højere version af Linux distro. Prøv venligst X11 desktop eller skift dit OS."),
        ("JumpLink", "JumpLink"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Vælg venligst den skærm, der skal deles (Betjen på modtagersiden)."),
        ("Show RustDesk", "Vis RustDesk"),
        ("This PC", "Denne PC"),
        ("or", "eller"),
        ("Continue with", "Fortsæt med"),
        ("Elevate", "Elevér"),
        ("Zoom cursor", "Zoom markør"),
        ("Accept sessions via password", "Acceptér sessioner via adgangskode"),
        ("Accept sessions via click", "Acceptér sessioner via klik"),
        ("Accept sessions via both", "Acceptér sessioner via begge"),
        ("Please wait for the remote side to accept your session request...", "Vent venligst på at fjernklienten accepterer din sessionsforespørgsel..."),
        ("One-time Password", "Engangskode"),
        ("Use one-time password", "Brug engangskode"),
        ("One-time password length", "Engangskode længde"),
        ("Request access to your device", "Efterspørg adgang til din enhed"),
        ("Hide connection management window", "Skjul forbindelseshåndteringsvindue"),
        ("hide_cm_tip", ""),
        ("wayland_experiment_tip", ""),
        ("Right click to select tabs", "Højreklik for at vælge faner"),
        ("Skipped", "Sprunget over"),
        ("Add to address book", "Tilføj til adressebog"),
        ("Group", "Gruppe"),
        ("Search", "Søg"),
        ("Closed manually by web console", "Lukket ned manuelt af webkonsollen"),
        ("Local keyboard type", "Lokal tastatur type"),
        ("Select local keyboard type", "Vælg lokal tastatur type"),
        ("software_render_tip", ""),
        ("Always use software rendering", "Brug altid software rendering"),
        ("config_input", ""),
        ("config_microphone", ""),
        ("request_elevation_tip", ""),
        ("Wait", "Vent"),
        ("Elevation Error", "Elevationsfejl"),
        ("Ask the remote user for authentication", "Spørg fjernbrugeren for godkendelse"),
        ("Choose this if the remote account is administrator", "Vælg dette hvis fjernbrugeren er en administrator"),
        ("Transmit the username and password of administrator", "Send brugernavnet og adgangskoden på administratoren"),
        ("still_click_uac_tip", ""),
        ("Request Elevation", "Efterspørger elevation"),
        ("wait_accept_uac_tip", ""),
        ("Elevate successfully", "Elevation lykkedes"),
        ("uppercase", "store bogstaver"),
        ("lowercase", "små bogstaver"),
        ("digit", "ciffer"),
        ("special character", "specielt tegn"),
        ("length>=8", "længde>=8"),
        ("Weak", "Svag"),
        ("Medium", "Mellem"),
        ("Strong", "Stærk"),
        ("Switch Sides", "Skift sider"),
        ("Please confirm if you want to share your desktop?", "Bekræft venligst, om du vil dele dit skrivebord?"),
        ("Display", "Visning"),
        ("Default View Style", "Standard visningsstil"),
        ("Default Scroll Style", "Standard rulle stil"),
        ("Default Image Quality", "Standard billedkvalitet"),
        ("Default Codec", "Standard codec"),
        ("Bitrate", "Bitrate"),
        ("FPS", "FPS"),
        ("Auto", "Auto"),
        ("Other Default Options", "Andre standardindstillinger"),
        ("Voice call", "Stemmeopkald"),
        ("Text chat", "Tekstchat"),
        ("Stop voice call", "Stop stemmeopkald"),
        ("relay_hint_tip", ""),
        ("Reconnect", "Genopret"),
        ("Codec", "Codec"),
        ("Resolution", "Opløsning"),
        ("No transfers in progress", "Ingen overførsler i gang"),
        ("Set one-time password length", "Sæt engangsadgangskode længde"),
        ("RDP Settings", "RDP indstillinger"),
        ("Sort by", "Sortér efter"),
        ("New Connection", "Ny forbindelse"),
        ("Restore", "Gendan"),
        ("Minimize", "Minimér"),
        ("Maximize", "Maksimér"),
        ("Your Device", "Din enhed"),
        ("empty_recent_tip", ""),
        ("empty_favorite_tip", ""),
        ("empty_lan_tip", ""),
        ("empty_address_book_tip", ""),
        ("eg: admin", "fx: admin"),
        ("Empty Username", "Tom brugernavn"),
        ("Empty Password", "Tom adgangskode"),
        ("Me", "Mig"),
        ("identical_file_tip", ""),
        ("show_monitors_tip", ""),
        ("View Mode", ""),
        ("login_linux_tip", ""),
        ("verify_rustdesk_password_tip", ""),
        ("remember_account_tip", ""),
        ("os_account_desk_tip", ""),
        ("OS Account", ""),
        ("another_user_login_title_tip", ""),
        ("another_user_login_text_tip", ""),
        ("xorg_not_found_title_tip", ""),
        ("xorg_not_found_text_tip", ""),
        ("no_desktop_title_tip", ""),
        ("no_desktop_text_tip", ""),
        ("No need to elevate", ""),
        ("System Sound", ""),
        ("Default", ""),
        ("New RDP", ""),
        ("Fingerprint", ""),
        ("Copy Fingerprint", ""),
        ("no fingerprints", ""),
        ("Select a peer", ""),
        ("Select peers", ""),
        ("Plugins", ""),
        ("Uninstall", ""),
        ("Update", ""),
        ("Enable", ""),
        ("Disable", ""),
        ("Options", ""),
        ("resolution_original_tip", ""),
        ("resolution_fit_local_tip", ""),
        ("resolution_custom_tip", ""),
        ("Collapse toolbar", ""),
        ("Accept and Elevate", ""),
        ("accept_and_elevate_btn_tooltip", ""),
        ("clipboard_wait_response_timeout_tip", ""),
        ("Incoming connection", ""),
        ("Outgoing connection", ""),
        ("Exit", ""),
        ("Open", ""),
        ("logout_tip", ""),
        ("Service", ""),
        ("Start", ""),
        ("Stop", ""),
        ("exceed_max_devices", ""),
        ("Sync with recent sessions", ""),
        ("Sort tags", ""),
        ("Open connection in new tab", ""),
        ("Move tab to new window", ""),
        ("Can not be empty", ""),
        ("Already exists", ""),
        ("Change Password", ""),
        ("Refresh Password", ""),
        ("ID", ""),
        ("Grid View", ""),
        ("List View", ""),
        ("Select", ""),
        ("Toggle Tags", ""),
        ("pull_ab_failed_tip", ""),
        ("push_ab_failed_tip", ""),
        ("synced_peer_readded_tip", ""),
        ("Change Color", ""),
        ("Primary Color", ""),
        ("HSV Color", ""),
        ("Installation Successful!", ""),
        ("Installation failed!", ""),
        ("Reverse mouse wheel", ""),
        ("{} sessions", ""),
        ("scam_title", ""),
        ("scam_text1", ""),
        ("scam_text2", ""),
        ("Don't show again", ""),
        ("I Agree", ""),
        ("Decline", ""),
        ("Timeout in minutes", ""),
        ("auto_disconnect_option_tip", ""),
        ("Connection failed due to inactivity", ""),
        ("Check for software update on startup", ""),
        ("upgrade_rustdesk_server_pro_to_{}_tip", ""),
        ("pull_group_failed_tip", ""),
        ("Filter by intersection", ""),
        ("Remove wallpaper during incoming sessions", ""),
        ("Test", ""),
        ("display_is_plugged_out_msg", ""),
        ("No displays", ""),
        ("elevated_switch_display_msg", ""),
        ("Open in new window", ""),
        ("Show displays as individual windows", ""),
        ("Use all my displays for the remote session", ""),
        ("selinux_tip", ""),
        ("Change view", ""),
        ("Big tiles", ""),
        ("Small tiles", ""),
        ("List", ""),
        ("Virtual display", ""),
        ("Plug out all", ""),
        ("True color (4:4:4)", ""),
        ("Enable blocking user input", ""),
        ("id_input_tip", ""),
        ("privacy_mode_impl_mag_tip", ""),
        ("privacy_mode_impl_virtual_display_tip", ""),
        ("Enter privacy mode", ""),
        ("Exit privacy mode", ""),
        ("idd_not_support_under_win10_2004_tip", ""),
        ("switch_display_elevated_connections_tip", ""),
        ("input_source_1_tip", ""),
        ("input_source_2_tip", ""),
        ("capture_display_elevated_connections_tip", ""),
        ("Swap control-command key", ""),
        ("swap-left-right-mouse", ""),
        ("2FA code", ""),
        ("More", ""),
        ("enable-2fa-title", ""),
        ("enable-2fa-desc", ""),
        ("wrong-2fa-code", ""),
        ("enter-2fa-title", ""),
        ("Email verification code must be 6 characters.", ""),
        ("2FA code must be 6 digits.", ""),
        ("Multiple Windows sessions found", ""),
        ("Please select the session you want to connect to", ""),
    ].iter().cloned().collect();
}
