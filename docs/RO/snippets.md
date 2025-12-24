# Hello world!

```
function main() -> () {
    println("Hello, world!")
}
```
# Variabile și constante
```
mut int32 x = 21 // o variabila mutabila de tip primitiv int32
const String name = "Filip" // o variabila nemutabila de tip referinta String
const int32 y = x // copiere
const String nameRef = name // aceeasi referinta ca si name
```

# Structuri de control (if/else)
```
mut bool isOk = true
if (isOk) {
    println("Totul e ok!")
} else {
    println("Ceva a mers gresit")
}
```

# Bucle (for, while)
```
for i = 0; i < 31; ++i {
    println("Iteratia ${i}")
}

```

# Funcții (parametri și return)

```
// Functie expresie (one liner only)
function add(int32 x, int32 y) = x + y

// Functie clasica
function mul(int32 x, int32 y) {
    return x * y
}

// Functie lambda
const Function<int32, int32, int32> sub = function (int32 x, int32 y) -> x - y
// Mai rapid, cu type inference
const auto sub = function (int32 x, int32 y) -> x - y
```

# Structuri de date (array, map, struct/obiect)
```
// Array static cu 5 elemente
float64[5] arr1 = {1, 2, 3, 4, 5}
println("Lungimea arrayului este: {arr1.length}")

// Array pe heap cu 5 elemente
float64[] arr2 = new float64[5]
// Sau mai rapid
float64[] arr2 = new [5]
// Saaaau
let arr2 = new float64[5]

// Indexarea incepe de la 0, ca in orice limbaj adevarat (sorry lua)
println(arr1[0] + arr1[1]) // Output: 3
```

# OOP / Interfețe / Trait-uri
```
interface Speaker {
    talk()
}

class Human implements Speaker {

    String name { pub get }

    pub Human(String name) {
        this.name = name
    }

    pub impl talk() {
        println("Hi!")
    }

    pub method work() {
        println("{this.name} is working!")
    }

}

function main() {
    Speaker person1 = Human("Andrei")
    person1.talk() // Output: Hi!
    if person1 is Human {
        Human h = (Human) person1
        h.work() // Output: Andrei is working!
    }
}
```

# Error handling

# Module și importuri

# Exemplu complet minimal