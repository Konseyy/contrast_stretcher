Programma izveidota valodā Rust
Repozitorija pieejama https://github.com/Konseyy/contrast_stretcher

Atverot .exe failu, jāievada relatīvs path uz bildi, ko vēlas mainīt, repozitorijā pievienoju arī
testēšanas nolūkos bildi test.gif, attiecīgi atverot programmu, pieņemot ka bilde atrodas vienā folderī ar programmu
tās vietu var norādīt vienkārši kā "test.gif"

Otrais prasītais ievades parametrs ir kontrasta vērtība. Atļautās vērtības ir robežās no -100 līdz 100 (ieskaitot), kur lielākas vērtības noved pie lielāka kontrasta

Pēc programmas izpildes tiek izveidots jauns folderis "images", kur tiek saglabāta bilde "comparison.png", kuras kreisajā pusē ir redzams oriģinālais fails, labajā jaunais