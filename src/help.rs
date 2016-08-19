use constants;

pub fn help() -> &'static str
{
    "

    Argumenti:

        -help

        -about

        -w -h
                Širina i/ili visina slike u pikselima.
                Dovoljno je upotrijebiti samo jedan argument,
                slici ća biti promijenjena veličina zadržavajući porporcije.
                Ukoliko se upotrijebe oba argumenta tada će se argumnet: 
                -w korisiti kada je širina slike veća od visine width > height 
                -h korisiti kada je visina slike veća od širine height > width

                • Upotreba: -w:640 i/ili -h:330

                Ukoliko se ne upotrijebi niti jedan argument
                tada će vrijednost za širinu biti 1280, a visinu 720.
        ----------------------------------------------------------------------------
        -f 
                Filter koji će se koristiti prilikom promijene veličine slike.
                Vrijednosti za filter su:
                    n -> Nearest, Nearest Neighbor
                    t -> Triangle, Linear Filter
                    c -> CatmullRom, Cubic Filter (dobar omjer brzine i kvalitete)
                    g -> Gaussian, Gaussian Filter
                    l -> Lanczos3, Lanczos with window 3 

                • Upotreba: -f:c

                Ukoliko se ne upotrijebi argument za filter,
                tada će se koristiti Nearest Neighbor.
        ----------------------------------------------------------------------------
        -if 
                Format (Image Format) u kojem će slike biti spremljene. 
                Podržana su dva formata:
                    jpg -> slike će biti spremljene u JPEG formatu
                    png -> slike će biti spremljene u PNG formatu

                • Upotreba: -if:png

                Ukolike se ne upotrijebi argument za format, 
                slike će biti spremljene u JPG.
        ----------------------------------------------------------------------------        
        -r
                Načini na koji će se mijenjati veličina forografije.
                    0 -> slici NEĆE biti promjenjena veličina. 
                         Može se upotrijebiti za konverziju iz jednog
                         formata u drugi.
                    1 -> slika će biti UMANJENA , ali ne i uvećana  
                         ako je manja od zadane vrijednosti.
                    2 -> slika će bit UVEĆANA, al ne i umanjena 
                         ako je veća od zadane vrijednosti.
                    3 -> slika će biti umanjena ili uvećana.

                • Upotreba: -r:3

                Ako nije zadan argumnet za smanjivanje upotrijebit će se metoda 1                
        ----------------------------------------------------------------------------
        -mpi      
                Broj slika koje će se odjednom učitati i paralelno obrađivati.
                Ne smije se pretjerati sa brojem odjednom učitanih slika, 
                jer ova vrijednost ovisi o veličini fotografija i raspoloživoj 
                memoriji računala.

                • Upotreba: -mpi:8

                Ukoliko se ne upotrijebi argument za \"max parallel image\",
                upotrijebit će se defaultna vrijednost 5.

    
    Primjer:
                img_rsz.exe -w:1024 -f:l -mpi:2

                Slici će biti promijenjena širina na 1024px, 
                visina proprcionalno širini, 
                pri čemu će se koristiti Lanczos3 filter i
                i učitavat će se u serijama po 5 slika.

    "    
}

pub fn about() {
    println!("\n • Made in Rust by {}\n", constants::AUTHOR);
}