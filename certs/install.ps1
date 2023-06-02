$certPath = "C:\Path\to\Certificates"  # Specify the path where your .crt certificates are located

$certFiles = Get-ChildItem -Path $certPath -Filter *.crt

$certStore = Get-Item -Path "Cert:\LocalMachine\Root"

foreach ($certFile in $certFiles) {
    $cert = New-Object System.Security.Cryptography.X509Certificates.X509Certificate2
    $cert.Import($certFile.FullName)

    if (!$certStore.Contains($cert)) {
        $certStore.Add($cert)
        Write-Host "Certificate $($cert.Subject) imported successfully."
    } else {
        Write-Host "Certificate $($cert.Subject) already exists in the store."
    }
}

