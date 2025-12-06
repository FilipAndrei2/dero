
# Disclaimer!
Acest proiect este intr-un stadiu foarte timpuriu de dezvoltare. Toate aspectele prezentate mai jos sunt subject to change, iar multe functionalitati inca nu sunt implementate. Recomand tratarea documentatiei curente ca si o declaratie a filosofiei limbajului, sau ca si o lectura usoara despre cum ar trebui sa arate limbajul de programare "perfect" din perspectiva unui pasionat de programare. Indiferent de orice, multumesc ca ai ales sa deschizi acest manual si sper sa te distrezi in continuare cel putin la fel de mult cat m-am distrat eu dezvoltand Dero. Have fun! 🙂

Filip Andrei
# Dero

## Table of contents

1. Introducere
   - Ce este limbajul?
   - Filosofie / paradigme
2. Instalare
   - Build
   - CLI / IDE
4. Sintaxă
   - Hello world!
   - Variabile, tipuri, expresii
5. Control flow
   - if/else, loops, pattern matching
6. Funcții și metode
7. Structuri/Clase / tipuri de date
8. Module și namespace
9. Exemple complete
10. Biblioteci standard
11. Concepte avansate
12. FAQ și troubleshooting
13. Referințe și link-uri externe

### Introducere

#### Ce este limbajul?

    *Dero* este un limbaj de programare portabil, compilat, tipizat static, conceput sa incurajeze practici bune de scriere de cod si sa asigure o experienta de dezvoltare a aplicatiilor prietenoasa.

    ##### *Poliglot!*
    Limbajul a fost conceput sa se integreze nativ cu alte limbaje de programare, fiind capabil sa vizeze mai multe platforme. Dero se integreaza perfect cu limbaje precum C, C++, Rust, Java, Kotlin si nu numai. Acelasi codebase scris in Dero poate fi compilat in executabil nativ, in JVM bytecode si/sau in JS, fara nevoia de a rescrie nimic.
    
    Dero a fost proiectat cu gandul sa incurajeze good practices out of the box (mai multe despre filozofia si quirkurile limbajului in curand 🙂), dar este capabil sa ofere libertatea de a scrie cod non-ideomatic, intr-un mod responsabil, daca proiectul tau cere asta.

    *Dero* pune la dispozitie dezvoltatorilor unelte precum:
    - Siguranta a memoriei prin dealocari automate, fara garbage collector
    - Incurajarea abstractizarilor zero-cost, pentru viteza turbo a programelor
    - O librarie standard cu multiple implementari (asigurand atributul poliglot al limbajului, dar si posibilitatea folosirii librariei standard Dero intr-un proiect scris intr-un limbaj de programare diferit, per-se)
    - Un Developer Kit oficial cu toate uneltele de dezvoltare necesare (build system, documentation generator, framework de testare si mocking)
    ... si multe altele in viitor! 😁
     
     
### Instalare
    [TBA]

### Sintaxa

    Sintaxa limbajului a fost aleasa sa fie simpla, moderna si distractiva. Sintaxa inspira familiaritate prin folosirea de constructii din alte limbaje populare contemporane.

#### Hello, world!

    ```dero
    function main() -> () {
        // This is a comment
        println("Hello, world!") // Look ma, no semicolons!
    }
    ```
    ..intuitiv, nu-i asa? 🙂

#### Variabile si Mutabilitate

    Urmatoarea sintaxa este folosita pentru a declara o variabila

    [tip de date] [identificator] = [valoare initiala]

    Poti omite initializarea, dar variabila va fi initializata implicit in functie de tipul ei

    ```dero
    function main() -> () {
        uint32 num = 2004 // immutable, primitive, stored on the stack
        String name = "Filip" // immutable, reference type, stored on the heap
        mut int32 coolNumber = 7 // mutable, primitive, stored on the stack
        
    }
    ```