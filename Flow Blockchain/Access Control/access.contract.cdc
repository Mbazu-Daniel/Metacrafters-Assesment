pub contract SomeContract {
    pub var testStruct: SomeStruct

    pub struct SomeStruct {

        //
        // 4 Variables
        //

        pub(set) var a: String

        pub var b: String

        access(contract) var c: String

        access(self) var d: String
// priv
        //
        // 3 Functions
        //

        pub fun publicFunc() {}

        access(contract) fun contractFunc() {}

        access(self) fun privateFunc() {}


        pub fun structFunc() {
            /**************/
            /*** AREA 1 ***/
            /**************/


        }

        init() {
            self.a = "a"
            self.b = "b"
            self.c = "c"
            self.d = "d"
        }
    }

    pub resource SomeResource {
        pub var e: Int

        pub fun resourceFunc() {
            /**************/
            /*** AREA 2 ***/
            /**************/

        }

        init() {
            self.e = 17
        }
    }

    pub fun createSomeResource(): @SomeResource {
        return <- create SomeResource()
    }

    pub fun questsAreFun() {
        /**************/
        /*** AREA 3 ****/
        /**************/


    }

    init() {
        self.testStruct = SomeStruct()
    }
}

// variable a
//   read scope  : 1,2,3,4
//   write scope : 1,2,3,4

// variable b
//   read scope  : 1,2,3,4
//   write scope : 1

// variable c
//   read scope  : 1,2,3
//   write scope : 1

// variable d
//   read scope  : 1
//   write scope : 1

// variable e
//   read scope  : 1,2,3,4
//   write scope : 2

// function publicFunc
//   can be called AREA : 1,2,3,4

// function contractFunc
//   can be called AREA : 1,2,3

// function privateFunc
//   can be called AREA : 1
