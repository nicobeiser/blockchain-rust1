//comando para el html del tarpaulin: cargo tarpaulin --target-dir src/coverage --skip-clean --exclude-files=target/debug/* --out html

#![cfg_attr(not(feature = "std"), no_std, no_main)]
pub use self::ClubSemRust::ClubSemRustRef;

#[ink::contract]
pub mod ClubSemRust {

    ///El trait ConversionFecha se encarga de facilitar el trabajo en el manejo del tiempo con milisegundos.
    ///Permite la traducción de una manera sencilla del Timestamp del entonrno a las distintas medidas de tiempo que
    ///puedan ser necesarias al momento de manejar el tiempo.   
    pub trait ConversionFecha {
        // Es llamado por el tipo que implemente el trait y devuelve los n segundos correspondientes
        fn from_segundos (&self) -> u64;
        // Es llamado por el tipo que implemente el trait y devuelve los n minutos correspondientes
        fn from_horas (&self) -> u64;
        // Es llamado por el tipo que implemente el trait y devuelve las n horas correspondientes
        fn from_dias (&self) -> u64;
        // Es llamado por el tipo que implemente el trait y devuelve los n dias correspondientes
        fn from_semanas (&self) -> u64;
        // Es llamado por el tipo que implemente el trait y devuelve las n semanas correspondientes
        fn from_meses (&self) -> u64;
        // Es llamado por el tipo que implemente el trait y devuelve los n años correspondientes
        fn from_anios (&self) -> u64;
    }

    /// Implementacion del trait ConversionFecha para el u64, mismo tipo de dato del Timestamp del entorno
    /// para realizar todas las operaciones necesarias. A partir del calculo de los minutos hacia las unidades mas significativas
    /// se decidió reutilizar la funcion from_segundos para mejorar la legibilidad de las formulas empleadas. Se adapta la formula para el caso de la conversion
    /// a meses y años, debido a que existen diferentes cantidades para las mismas unidades (28, 29, 30 o 31 dias para 1 mes y diferentes cantidades de dias 
    /// en la composicion de 1 año).
    
    impl ConversionFecha for u64 {
        /// Se multiplica el numero que llama el metodo por 1000, devolviendo el resultado 
         fn from_segundos (&self) -> u64 {
             return self * 1000;
         }   
          
         ///Se multiplica el resultado de los segundos correspondientes
         ///por 3600 se obtiene la cantidad de horas desde el numero disparador del metodo. 
         fn from_horas (&self) -> u64 {
             return self.from_segundos() * 3600;
         }
          
         ///Se multiplica el resultado de los segundos correspondientes
         ///por 86400 se obtiene la cantidad de dias desde el numero disparador del metodo.
         fn from_dias (&self) -> u64 {
             return self.from_segundos() * 86400;
         }
            
         ///Se multiplica el resultado de los segundos correspondientes
         ///por 86400 se obtiene la cantidad de semanas desde el numero disparador del metodo.
         fn from_semanas (&self) -> u64 {
             return self.from_segundos() * 604800;
         }
         
         ///Se multiplica el numero disparador del metodo por 2629743, devolviendo el numero de meses correspondiente donde 1 Mes = 30.44 días 
         fn from_meses (&self) -> u64 {
             return self.from_segundos() * 2629743;
         }
         
         ///Se multiplica el numero disparador del metodo por 2629743, devolviendo el numero de años correspondiente donde 1 Año = 365.24 días 
         fn from_anios (&self) -> u64 {
             return self.from_segundos() * 31556926;
         }
     }

    use ink::env::block_timestamp;
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;

    #[derive(scale::Decode, scale::Encode, Debug, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]

    ///Este struct, se encargar de almacenar los costos monetarios relacionados al Club. Almacena:
    /// :> El costo para categoria (a, b y c), las cuales son un tipo de dato u128.
    /// :> La cantidad de pagos consecutivos para un descuento ofrecido por el Club (beneficio), el cual es de un tipo de
    ///    dato u128.
    /// :> El monto del beneficio, el cual es proporcionado cuándo el socio acumula la cantidad de pagos consecutivos
    ///    mencionados, el cual es de un tipo de dato u8. 
    /// Incluye 5 metodos: new, set_costo, get_costo, set_beneficio, y set_pagos_consecutivos_para_beneficio.
    pub struct CostosCategoria {
        a: u128,
        b: u128,
        c: u128,
        beneficio: u128,
        pagos_consecutivos_para_beneficio: u8,
    }
    
    impl CostosCategoria {
        ///Recibe el valor del costo de la categoria a, b, y c en variables de tipo u128, luego recibe el
        ///valor a descontar una vez que sea bonificado y luego recibe la cantidad de pagos consecutivos que 
        ///deberian concretarse para aplicarle beneficio al proximo pago
        fn new(val_a: u128, val_b: u128, val_c: u128, valor_beneficio: u128, pagos_consecutivos_para_beneficio: u8) -> CostosCategoria {
           CostosCategoria { a: (val_a), b: (val_b), c: (val_c), beneficio: (valor_beneficio), pagos_consecutivos_para_beneficio} 
        }

        ///Recibe en el parametro categoria, una variable de tipo CategoriasSocios
        ///la cual usa para matchear con las categorias del struct CostosCategoria
        ///y actualizar el valor de la variable con el valor ingresado en el parametro
        ///nuevo_valor
        fn set_costo (&mut self, nuevo_valor: u128, categoria: &CategoriasSocios) -> bool {
            match categoria {
                CategoriasSocios::A => {self.a = nuevo_valor; return true},
                CategoriasSocios::B => {self.b = nuevo_valor; return true},
                CategoriasSocios::C => {self.c = nuevo_valor; return true},
            }
        }

        ///Recibe por parametro una CategoriasSocios la cual usa para matcheaer con
        ///las variables del CostosCategoria, si CategoriasSocios es A retorna el contenido de
        ///la variable a, si es B, retorna el contenido de la variable b si es C retorna
        ///el contenido de la variable c
        fn get_costo (&self, categoria: &CategoriasSocios) -> u128 {
            match categoria {
                CategoriasSocios::A => self.a,
                CategoriasSocios::B => self.b,
                CategoriasSocios::C => self.c,
            }
        }

        ///Recibe el valor del nuevo beneficio por parametro y lo setea en la
        ///variable beneficio
        fn set_beneficio (&mut self, nuevo_benedificio: u128) -> bool {
            self.beneficio = nuevo_benedificio;
            return true;
        }

        ///Recibe el nuevo valor para los pagos consecutivos para obtener el beneficio
        ///por parametro y lo setea en la variable pagos_consecutivos_para_beneficio
        fn set_pagos_consecutivos_para_beneficio (&mut self, nuevo_valor: u8) -> bool {
            self.pagos_consecutivos_para_beneficio = nuevo_valor;
            return true;
        }
    }

    #[derive(scale::Decode, scale::Encode, Debug, Clone, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    /// Este struct almacena toda la informacion perteneciente a un pago. Almacena:
    /// :> El DNI correspondiente al socio al que pertenece el pago (dni), el cual es un tipo de dato u32.
    /// :> El monto correspondiente al pago (monto), el cual es un tipo de dato u128.
    /// :> La fecha de vencimiento del pago (fecha_venci), el cual es un tipo de dato u64.
    /// :> La fecha de pago, del pago (fecha_pago), el cual es un tipo de dato Option(u64), el cual si es None indica que
    ///    el pago no fue efectuado, en cambio si hay un Some() indica que el pago fue efectuado en la fecha especificada.
    /// :> Si el pago fue bonificado, el cual es un tipo de dato bool, el cual si es false, indica que el pago no fue 
    ///    bonificado, y si fue bonificado, esta variable almacena un true.
    /// 
    /// Cuenta con 6 implementaciones: new, get_dni_socio, get_monto, get_fecha_venci, get_fecha_pago, y set_fecha_pago. 
    pub struct Pago {
        dni: u32,
        monto: u128,
        fecha_venci: u64,
        fecha_pago: Option<u64>,
        bonificado: bool,
    }

    impl Pago {
        ///Recibe un dni de tipo u32, un monto de tipo u128, una fecha_vencimiento(expresada en milisegundos) de tipo u64 un Option de u64 para la fecha de pago
        ///ya que podria haber pagado o no y por ultimo un booleano que indica si el pago ha sido bonificado o no
        pub fn new (dni: u32, monto: u128, fecha_venci: u64, fecha_pago: Option<u64>, bonificado: bool) -> Pago {
            Pago {dni, monto, fecha_venci, fecha_pago, bonificado}
        }
        ///Retorna el contenido de la variable dni
        pub fn get_dni_socio (&self) -> u32 {
            return self.dni;
        }
        ///Retorna el contenido de la variable monto
        pub fn get_monto (&self) -> u128 {
            return self.monto;
        }
        ///Retorna el contenido de la variable fecha_venci
        pub fn get_fecha_venci (&self) -> u64 {
            return self.fecha_venci;
        }
        ///Retorna el contenido de la variable fecha_pago
        pub fn get_fecha_pago (&self) -> Option<u64> {
            return self.fecha_pago;
        }
        ///Setea el valor ingresado como parametro, en la variable fecha_pago
        pub fn set_fecha_pago (&mut self, fecha_pago: u64) {
            self.fecha_pago = Some(fecha_pago);
        }
    }

    /// Se trata de un Enum que contiene las variantes disponibles de categorías de socios, las cuales al momento son
    /// A, B o C. Cuenta con una implementacion:
    /// categoria_from_id.
    #[derive(scale::Decode, scale::Encode, Debug, Clone, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum CategoriasSocios {
        A,
        B,
        C
    }

    impl CategoriasSocios {
        /// Se encarga de obtener una categoria en base a un ID recibido como parametro, el cual es de un tipo de dato u32
        /// (id). En base a este ID, resuelve que: si el ID es 1, es categoria A; si es 2, es categoria B; si es 3, es 
        /// categoria C; y si no es 1, 2 o 3, arroja un panic informando que el ID no es valido. De esta forma, devuelve un
        /// Option(CategoriasSocios), el cual si es un ID reconocible, devuelve Some(CategoriasSocios), de lo contrario, 
        /// arroja un panic informando lo sucedido.
        pub fn categoria_from_id(id: &u32) -> Option<CategoriasSocios> {
            match id {
                1 => Some(CategoriasSocios::A),
                2 => Some(CategoriasSocios::B),
                3 => Some(CategoriasSocios::C),
                _ => panic!("El id de categoria ingresado no es valido!"),
            }
        }
    }

    #[derive(scale::Decode, scale::Encode, Debug, Clone, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    //Enum que representa las variantes de actividades deportivas ofrecidas por el Club SemRust.
    //Se trabaja con un Id para cada uno de los valores de la estructura.
    pub enum Deporte {
        Futbol,
        Basquet,
        Rugby,
        Hockey,
        Natacion,
        Tenis,
        Paddle,
        Gimnasio,
    }

    impl Deporte{
        ///El método deporte_from_id lleva a cabo la interpretación de los diferentes valores de el Id de deportes.
        ///Se recibe un Option<u32> como id, ya que se conoce que existen categorias sin la información explícita de un deporte.
        ///El disparador del metodo recibirá un valor Option<Deporte> una vez finalizada la ejecución de la función. 
        ///Dentro del método se trabaja el Option<u32>, en el caso de recibir un valor None se devuelve el mismo, este caso aplica
        ///para aquellos socios que no necesiten información explícita sobre el campo deporte. Por otro lado, dadas las circunstancias de recibir
        ///un valor Some(u32) que se encuentre dentro del rango disponible de Ids, deporte_from_id, atraves de una estructura match, encuentra la 
        ///variante respectiva y la retorna como Option<Deporte> envolviendo el valor en Some(). En la ultima eventualidad de haber recibido un Some(u32)
        ///con un número fuera del rango correcto de Ids se ejecuta el macro panic! deteniendo la ejecución dada la imposibilidad de seguir con un error.
        pub fn deporte_from_id(option_id: &Option<u32>) -> Option<Deporte> {
            if let Some (id) = option_id {
                match id {
                    1 => return Some (Deporte::Futbol),
                    2 => return Some (Deporte::Basquet),
                    3 => return Some (Deporte::Rugby),
                    4 => return Some (Deporte::Hockey),
                    5 => return Some (Deporte::Natacion),
                    6 => return Some (Deporte::Tenis),
                    7 => return Some (Deporte::Paddle),
                    8 => return Some (Deporte::Gimnasio),
                    _ => return panic!("El ID de deporte no es valido!"),
                }
            }
            else {
                return None;
            }
        }
    }

    #[derive(scale::Decode, scale::Encode, Debug, Clone, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    /// Se trata de un Struct que almacena los datos correspondientes al socio, los cuales son;
    /// :> Apellido y Nombre del socio, el cual es un tipo de dato String (apellido_y_nombre);
    /// :> DNI del socio, el cual es un tipo de dato u32 (dni);
    /// :> La categoría elegida por el socio, la cual es una variante del Enum CategoriasSocios (categoria);
    /// :> El deporte elegido por el socio (en caso de haber elegido la variente B del Enum CategoriasSocios), el cual 
    ///    es un dato de tipo Option(Deporte) (deporte);
    /// Cuenta con 5 implementaciones: new, get_appellido_y_nombre, get_dni, get_categoria, get_deporte;
    pub struct Socio {
        apellido_y_nombre: String,
        dni: u32,
        categoria: CategoriasSocios,
        deporte: Option<Deporte>,
    }

    impl Socio {
        /// Se encarga de crear un socio y devolverlo: Recibe;
 	    /// :> Un tipo de dato String que corresponde al apellido y nombre del socio (apellido_y_nombre);
        /// :> Un tipo de dato u32 que corresponde al DNI del socio (dni); 
        /// :> Una variante del tipo de dato CategoriasSocios, correspondiente a la categoria del socio (categoria);
        /// :> Un tipo de dato Option(Deporte), el cual contiene el deporte escogido por el socio. Este se guardara como
        ///    None si la categoria es A o C, y si la categoría es B, guardara lo que contenga su correspondiente parametro,
        ///    que puede ser None o una variante del tipo de dato Deporte (deporte).
		pub fn new (apellido_y_nombre: String, dni: u32, categoria: CategoriasSocios, deporte: Option<Deporte>) -> Socio {
            match categoria {
                CategoriasSocios::B => Socio {dni, categoria, apellido_y_nombre, deporte},
                   _ => Socio {dni, categoria, apellido_y_nombre, deporte: (None)},
            }
        }

        /// Se encarga de clonar el contenido del campo apellido_y_nombre, y devolverlo, por lo que devuelve
        /// un tipo de dato String. Recibe una referencia a si mismo (Socio).
        pub fn get_apellido_y_nombre (&self) -> String {
            return self.apellido_y_nombre.clone();
        }

        /// Se encarga de copiar el contenido del campo dni, y devolverlo, por lo que devuelve un tipo de dato u32.
        /// Recibe una referencia a si mismo (Socio).
        pub fn get_dni (&self) -> u32 {
            return self.dni;
        }

        /// Se encarga de clonar el contenido del campo categoria, y devolverlo, por lo que devuelve una variante del 
        /// tipo de dato CategoriasSocios. Recibe una referencia a si mismo (Socio).
        pub fn get_categoria (&self) -> CategoriasSocios {
            return self.categoria.clone();
        }

        /// Se encarga de clonar el contenido del campo deporte, y devolverlo, por lo que devuelve un Option(Deporte), que
        /// será None si la categoria es A o C, y si la categoría es B, guardara lo que contenga el campo,
        /// que puede ser None o una variante del tipo de dato Deporte (deporte). Recibe una referencia a si mismo (Socio).
        pub fn get_deporte (&self) -> Option<Deporte> {
            return self.deporte.clone();
        }
    }

    /// Este struct contiene toda la información relevante al Club, por lo cual, es el storage del Club. Almacena:
    /// :> Los costos referentes al club (costos), el cual es un tipo de dato CostosCategoria.
    /// :> Los socios del club (socios), el cual es un tipo de dato Vec(Socio).
    /// :> Los pagos efectuado por el Club (pagos), el cual es un tipo de dato Vec(Pago).
    /// :> El owner del Club, y por lo tanto del contrato (owner), el cual es un tipo de dato AccountId.
    /// :> Los permitidos, o staff's, para operar en el contrato en los metodos en que están permitidos operar (permitidos),
    ///    el cual es un tipo de dato Vec(AccountId).
    /// :> La politica de autorización (politica_activada), la cual indica que cualquiera pueda operar en el contrato en 
    ///    los metodos que no se refieren a la administracion de costos y permitidos del mismo, de estar desactivada. 
    ///    Es un tipo de dato bool, que si guarda false indica que la politica está desactivada, y cualquiera puede 
    ///    operar en los metodos mencionados, en cambio si esta en true, no podrá acceder cualquiera, solo un staff o el
    ///    propio owner.
    /// :> La fecha de emisión del último pago mensual (emision_ultimo_pago), el cual es un tipo de dato Option(u64). 
    ///    Este almacena None al momento de construir el club, ya que no hay pagos emitidos, y este guardará un Some() 
    ///    cuando se registre el primer socio, guardando la fecha en la que se registro dicho socio.
    /// 
    /// Cuenta con distintos métodos que permiten: la creacion del club, la actualizacion de cualquier valor de sus 
    /// costos, la verificacion del usuario que está llamando a los metodos, el registro de socios y staff's, cambio de
    /// owner, obtención y consulta de los pagos, corrobaración para el beneficio, la emisión de pagos y el registro de
    /// pagos, entre otros metodos.
    #[ink(storage)]
    pub struct ClubSemRust {
        costos: CostosCategoria,
        socios: Vec<Socio>,
        pagos: Vec<Pago>,
        owner: AccountId,
        permitidos: Vec<AccountId>,
        politica_activada: bool,
        emision_ultimo_pago: Option<u64>,
    }

    impl ClubSemRust {
        ///Recibe los 3 primeros costos de las categorias a,b y c en parametros de tip u128, recibe el valor del beneficio en un parametro de tipo u128
        ///y recibe los pagos consecutivos para aplicar beneficio en un parametro de tipo u8, luego, crea el Vec de socios vacio, el vec de pagos vacio
        ///la variable costos con todos los parametros ingresados previamente, setea al owner como el caller actual y crea el vec de permitidos vacio
        #[ink(constructor)]
        pub fn new(costo_a: u128, costo_b: u128, costo_c: u128, valor_beneficio: u128, pagos_consecutivos_para_beneficio: u8) -> Self {
            return ClubSemRust::new_priv(costo_a, costo_b, costo_c, valor_beneficio, pagos_consecutivos_para_beneficio);
        }

        fn new_priv(costo_a: u128, costo_b: u128, costo_c: u128, valor_beneficio: u128, pagos_consecutivos_para_beneficio: u8) -> Self {
            let socios = Vec::new();
            let pagos = Vec::new();
            let costos = CostosCategoria::new(costo_a, costo_b, costo_c, valor_beneficio, pagos_consecutivos_para_beneficio);
            let owner = ClubSemRust::env().caller();
            let permitidos = Vec::new();
            Self {socios, pagos, costos, owner, permitidos, emision_ultimo_pago: (None), politica_activada: (true)}
        }
        
        ///La función get_socios devuelve la lista "cruda" de socios registrados que almacena el club. Se recibe el parametro self (ClubSemRust)
        ///y se retorna una estructura Vec<Socio>. El metodo cumple con la politica de autorizacion verificando los permisos
        ///necesarios, devolviendo un panic en caso de no contar con los mismos. En caso de no existir socios se devuelve un vector vacio.
        #[ink(message)]
        pub fn get_socios (&self) -> Vec<Socio> {
            return self.get_socios_priv();
        }

        fn get_socios_priv (&self) -> Vec<Socio> {
            if self.verificar_permisos() {
                let mut vec_copia = Vec::new();
                vec_copia.clone_from(&self.socios);
                return vec_copia;
            }
            return panic!("No se cuenta con los permisos necesarios!");
        }

	    /// Este método, permite realizar un cambio de Owner del Club, sólo si el que realiza el llamado a este método,
        /// es el mismo Owner. Dicho cambio de owner, se realiza por el ingresado como parámetro (nuevo_owner). En caso
        /// que cualquier otro usuario realice esta operación, este método arrojará un panic!, abortando la operación 
        /// del método. El metodo recibe como parametros una referencia mutable de sí mismo (ClubSemRust), y el nuevo 
        /// owner, que es un tipo de dato AccountId.
        #[ink(message)]
        pub fn set_owner(&mut self, nuevo_owner: AccountId) -> bool {
            return self.set_owner_priv(nuevo_owner);
        }

        fn set_owner_priv(&mut self, nuevo_owner: AccountId) -> bool{
            if self.es_admin() {
                self.owner = nuevo_owner;
                return true;
            } 
            return panic!("No cuenta con rango Owner para realizar esta operacion!");
        }

        ///El get politica de autorizacion primero comprueba que el caller sea admin o staff y luego
        ///retorna el valor de la variable politica_activada, de no ser admin o staff arrojara un panic
        #[ink(message)]
        pub fn get_politica_autorizacion(&self) ->bool{
            return self.get_politica_autorizacion_priv();
        }


        fn get_politica_autorizacion_priv(&self) -> bool {
            if ((self.es_admin()) || (self.es_staff())) {
                return self.politica_activada;
            } 
            return panic!("No se cuenta con los permisos necesarios!");
        }

        ///La función toggle_politica_autorizacion activa y desactiva la política de autorización en la administración
        ///del ClubSemRust. Se aplica la operación logica NOT sobre el valor del booleano correspondiente a la política
        ///de autorización del club. Se recibe la referencia mutable del ClubSemRust y se retorna el nuevo estado de la 
        ///política. Es importante notar que solo el Adress con característica de Owner puede ejecutar exitosamente el 
        ///método, de no contar con los permisos requeridos se lanza un panic! notando el error.
        #[ink(message)]
        pub fn toggle_politica_autorizacion(&mut self) -> bool {
            return self.toggle_politica_autorizacion_priv();
        }

        fn toggle_politica_autorizacion_priv(&mut self) -> bool {
            if self.es_admin() {
                self.politica_activada = !self.politica_activada;
                return self.politica_activada;
            }
            return panic!("No cuenta con rango Owner para realizar esta operacion!");
        }

	    /// Este método, permite agregar un usuario permitido para operar, que lo definimos como "staff", solo si el
        /// método es llamado por el Owner, y si dicho staff no está cargado. De esta forma, si se cumplen las 
        /// condiciones descriptas anteriormente, el staff será agregado exitosamente, devolviendo un "true" como 
        /// demostracion. De lo contrario, si alguna de ellas no se cumple, el método arrojará un panic! con un mensaje
        /// informando la situacion. El método recibe como párametro una referencia mutable de sí mismo (ClubSemRust), y 
        /// el nuevo staff (nuevo_staff), que es un tipo de dato AccountId.
        #[ink(message)]
        pub fn agregar_staff(&mut self, nuevo_staff: AccountId) -> bool {
            return self.agregar_staff_priv(nuevo_staff);
        }

        fn agregar_staff_priv(&mut self, nuevo_staff: AccountId) -> bool {
            if (self.es_admin()) && (!self.permitidos.contains(&nuevo_staff)) {
                self.permitidos.push(nuevo_staff);
                return true;
            }
            return panic!("No cuenta con rango Owner o la cuenta ingresada ya es Staff!");
        }

       ///Primero se chequea que el caller sea el owner y que el accountId que se quiere quitar del staff
       ///exista en el vec de permitidos. Luego se obtiene la posicion del accountId que queremos quitar del 
       ///vec de permitidos y finalmente se remueve la posicion en la cual se encontro el account id
       ///de no ser owner o no encontrarse el accoundId, se arrojara un panick 
        #[ink(message)]
        pub fn quitar_staff(&mut self, quitar: AccountId) -> bool {
            return self.quitar_staff_priv(quitar);
        }

        fn quitar_staff_priv(&mut self, quitar: AccountId) -> bool {
            if (self.es_admin()) && (self.permitidos.contains(&quitar)) {
                let pos = self.permitidos.iter().position(|staff| staff == &quitar).unwrap();
                self.permitidos.remove(pos);
                return true;
            }
            return panic!("No se cuenta con rango Owner o la cuenta ingresada no es Staff!");
        }

        fn es_admin(&self)-> bool{
            return self.env().caller() == self.owner;
        }

	    /// Este método, permite realizar comprobaciones internas en el programa, verificando si el usuario que está
        /// llamando es reconocible como staff. De esta forma, si es reconocido como staff, devuelve "true", o "false"
        /// de lo contrario. El método recibe como párametro una referencia a si mismo (ClubSemRust).
        fn es_staff(&self) -> bool {
            return self.permitidos.contains(&self.env().caller());     
        }

       /// Para empezar se chequea que el caller sea admin o staff, luego, con los parametros recibidos(el id de categoria y el nuevo costo)
       /// se llama a la funcion set_costo de la variable costos del club, actualizando asi, el nuevo costo de una categoria en especifico
       /// recibida por parametro. para comprobar que la categoria sea una permitida se llama al metodo categoria_from_id
       /// la cual arrojara un panic si el id ingresado no es o 1 o 2 o 3. Por otra parte, si el caller no es admin o staff se arrojara un panic
        #[ink(message)] 
        pub fn actualizar_costos_categoria(&mut self, id_categoria : u32, nuevo_costo : u128,) -> bool {
            return self.actualizar_costos_categoria_priv(id_categoria, nuevo_costo);
        }

        pub fn actualizar_costos_categoria_priv(&mut self, id_categoria: u32, nuevo_costo: u128) -> bool {
            if (self.es_admin()) || (self.es_staff()) {

                if let Some(categoria) = CategoriasSocios::categoria_from_id(&id_categoria){
                    return self.costos.set_costo(nuevo_costo, &categoria);
                }
            }
            return panic!("No se cuenta con los permisos necesarios!");
        }

	    /// Este metodo, se encarga de verificar que se cumpla alguno de los 3 permisos para realizar una acción: que el
        /// usuario que llama al metodo sea owner, sea staff, o que la politica de autorizacion este desactivada. De 
        /// esta manera, realiza una verificacion entre las 3 condiciones: con que una condicion de ellas resultase 
        /// verdadera, la expresión resulta ser verdadera y devuelve "true", en cambio, si ninguna devuelve "true", la 
        /// expresión devolverá "false".
        /// 
        /// Este método recibe una referencia a si mismo (ClubSemRust), y devuelve un tipo de dato bool.
        #[ink(message)]
        pub fn verificar_permisos(&self) -> bool {
            return self.verificar_permisos_priv();
        }

        fn verificar_permisos_priv(&self) -> bool {
              return (self.es_admin()) || (self.es_staff()) || (!self.politica_activada);
        }

        ///Este método altera el valor actual del descuento aplicable para aquellos socios que cumplan las condiciones
        ///necesarias, recibiendo el nuevo importe y actualizando el importe del beneficio que el club ofrece. Esta función
        ///únicamente puede ser ejecutada por el usuario administrador o los integrantes del staff, cualquier otro usuario
        ///encontrará denegado el intento de actualización.
        #[ink(message)]
        pub fn actualizar_costo_beneficio(&mut self, nuevo_costo : u128) -> bool {
            return self.actualizar_costo_beneficio_priv (nuevo_costo);
        }

        fn actualizar_costo_beneficio_priv(&mut self, nuevo_costo : u128) -> bool {
            if (self.es_admin()) || (self.es_staff()) {
                return self.costos.set_beneficio(nuevo_costo);
            }

            return panic!("No se cuenta con los permisos necesarios!");
        }

	    /// Este método permite modificar la cantidad de pagos consecutivos necesarios para acceder a un beneficio. 
        /// Dicha modificación, será posible solamente si el usuario que realiza el llamado es Owner o Staff. De 
        /// esta forma, si el usuario cumple con uno de estos criterios, se realizará la actualizacion, y se devolverá
        /// un "true" como muestra de ello. De lo contrario, arrojará un panic! con un mensaje informando la situación.
        /// El metodo recibe como parametros una referencia mutable de sí mismo (ClubSemRust), y la nueva cantidad de
        /// pagos consecutivos necesarios para acceder al descuento (nuevo_valor), que es un tipo de dato u8.
        #[ink(message)]
        pub fn actualizar_pagos_consecutivos_para_descuento(&mut self, nuevo_valor: u8) -> bool {
            return self.actualizar_pagos_consecutivos_para_descuento_priv (nuevo_valor);
        }

        fn actualizar_pagos_consecutivos_para_descuento_priv(&mut self, nuevo_valor: u8) -> bool {
            if (self.es_admin()) || (self.es_staff()) {
                return self.costos.set_pagos_consecutivos_para_beneficio(nuevo_valor);
            }

            return panic!("No se cuenta con los permisos necesarios!");
        }

        ///Se comprueba que el caller sea admin o staff o que la politica de autorizacion este desactivada, luego
        ///se busca al socio en el vec de socios con un iter, y si el metodo find retorna un some, devuelve un clone
        ///del socio encontrado, sino retorna un None. Por otra parte, si el caller no es admin o staff y la politica esta activada
        ///el metodo arrojara un panick
        #[ink(message)]
        pub fn get_socio (&self, dni: u32) -> Option<Socio> {
            return self.get_socio_priv(dni);
        }

        fn get_socio_priv (&self, dni: u32) -> Option<Socio> {

            if self.verificar_permisos() {
                if let Some(socio) = self.socios.iter().find(|socio| socio.dni == dni) {
                    return Some (socio.clone());
                }
                else {
                    return None;
                }
            }
            return panic!("No se cuenta con los permisos necesarios o la politica de autorizacion se encuentra activada!");
        }

        ///El método existe_socio simplifica la verificación de la existencia de un socio, recibiendo el dni del 
        ///socio a verificar y enmascarando el método de get_socio retorna el booleano correspondiente a la 
        ///afirmación de la firma.
        fn existe_socio(&self, dni: u32) -> bool {
            if let Some (_) = self.get_socio(dni) {
                true
            }
            else {
                false
            }
        }
        
	    /// Este método, se encarga de realizar un resumen de los pagos de un socio, si se ingresa su DNI, de lo 
        /// contrario mostrará un resumen de todos los pagos de todos los socios. De esta forma, si el usuario que llama
        /// al método cumple con los permisos necesarios, se realizará, la verificación de si se ha ingresado un DNI o
        /// no, de lo contrario, el método arrojará un panic informando lo sucedido.
        /// 
        /// De haber ingresado un DNI, se obtiene el socio mediante su DNI, y se recorren todos los pagos (ya sea 
        /// un pago realizado o un pago pendiente), buscando todos aquellos pagos que contengan el DNI del socio, y se 
        /// extrae de ellos su monto y se agregan a un Vec. Una vez obtenidos todos los pagos de un socio, para devolver
        /// lo pedido, se crea una tupla de 2 elementos, la cual contiene como primer elemento al Socio con toda su info,
        /// y como segundo elemento al Vec obtenido anteriormente, con todos los pagos de dicho socio. Como el método
        /// esta planteado de forma que se pueda emitir una consulta por un socio en especial, o por todos los socios,
        /// este método devuelve un Vec de tuplas, donde cada tupla es igual a la descripta anteriormente, por lo tanto, 
        /// esta tupla obtenida se la inserta en un Vec, siendo este último, el tipo de dato que devuelve el metodo: 
        /// Vec((Socio, Vec(u128))). De no encontrar a un socio con el DNI ingresado, el método arrojará un panic 
        /// infromando lo sucedido.
        /// 
        /// Si se detecta que no se ha ingresado un DNI, se hace lo mencionado anteriormente para 
        /// un DNI ingresado, pero a cada socio, se le extrae el DNI para detectar sus pagos, y agregar el monto al Vec de 
        /// montos. Cuando se termina de procesar a un socio, se crea su tupla y se agrega al Vec de tuplas.
        /// 
        /// Este método, recibe una referencia a sí mismo (ClubSemRust), un tipo de dato Option de u32 (option_dni), y como destacamos 
        /// anteriormente, devuelve un tipo de dato Vec, que contiene tuplas, donde cada tupla, contiene un socio y otro
        /// Vec que contiene los montos: Vec((Socio, Vec(u128))).
        #[ink(message)]
        pub fn consulta_de_pagos(&self, option_dni: Option<u32>) -> Vec<(Socio, Vec<u128>)> { 
            return self.consulta_de_pagos_priv(option_dni);
        }

        fn consulta_de_pagos_priv(&self, option_dni: Option<u32>) -> Vec<(Socio, Vec<u128>)> {
            if self.verificar_permisos() {

                if let Some (dni) = option_dni {
                    if let Some (socio) = self.get_socio(dni) {
                        let mut vec_tuplas = Vec::new();
                        let mut vec_pagos = Vec::new();
                        for pago in &self.pagos {
                            if pago.dni == socio.dni {
                                vec_pagos.push(pago.monto);
                            }
                        }
                        let tupla = (socio.clone(), vec_pagos);
                        vec_tuplas.push(tupla);
                        return vec_tuplas;
                    }

                    return panic!("No se ha encontrado un socio con el DNI ingresado!")
                }
                else {
                    let mut vec_tuplas = Vec::new();
                    for socio in &self.socios {
                        let mut vec_pagos = Vec::new();
                        for pago in &self.pagos {
                            if pago.dni == socio.dni {
                                vec_pagos.push(pago.monto);
                            }
                        }
                        let tupla = (socio.clone(), vec_pagos);
                        vec_tuplas.push(tupla);
                    }
                    return vec_tuplas;
                }
            }
            return panic!("No se cuenta con los permisos necesarios o la politica de autorizacion se encuentra activada!");
        }

        ///Para empezar se comprueba que el caller sea admin o staff o que la politica de autorizacion este desactivada.
        ///luego crea un vec vacio, posteriormente, recorre el vec de pagos y chequea si en cada pago, el dni del pago es
        ///igual al ingresado, entonces se agrega al vec recientemente creado, por ultimo retorna el vec luego del for.
        ///Si el caller no tenia permisos necesarios para llamar a la funcion, entonces se arrojara un panick.
        ///Cabe aclarar que este metodo tiene una doble funcionalidad, si se quiere recibir los pagos totales de una persona en
        ///especifico, se recibira un Some(con un dni) y si se quiere recibir todos los pagos del club, se recibira un None como
        ///parametro y cuando en el if se detecte que el DNi es none, retornara los pagos totales del club
        #[ink(message)]
        pub fn get_pagos_totales (&self, dni: Option<u32>) -> Vec<Pago> {
            return self.get_pagos_totales_priv(&dni);
        }


        fn get_pagos_totales_priv (&self, dni : &Option<u32>) -> Vec<Pago>{
            if self.verificar_permisos() {
                let mut aux_vec = Vec::new();
                if let Some(dni) = dni{
                    for pago in &self.pagos{
                        if pago.dni == *dni{
                            aux_vec.push(pago.clone());
                        }
                    }
                } else {
                    aux_vec = self.pagos.clone();
                }
                return aux_vec;
            }
            return panic!("No se cuenta con los permisos necesarios o la politica de autorizacion se encuentra activada!");
        }

	    /// Este método se encarga de verificar si a un socio le corresponde la bonificación en un pago al momento de
        /// ejecutarlo. Dicha bonificacion se obtiene si: 
        /// :> Pago a tiempo los últimos pagos, donde la cantidad de pagos es probista por el club.
        /// :> Si dentro de la cantidad de pagos descripta anteriomente, no hubo ya un pago bonifcado.
        /// 
        /// Para comenzar, se obtiene todos los pagos realizados por un socio. Luego, se verifica si la cantidad de pagos
        /// obtenidos es mayor o igual a la cantidad de pagos necesarios para acceder al beneficio, ya que de no ser así,
        /// el socio no puede obtener el beneficio. Ya si hay tantos pagos (ya sea pendientes como realizados) como los
        /// necesarios para obtener el beneficio, se obtienen tantos últimos pagos, como indique la cantidad de 
        /// últimos pagos necesarios probista por el club y se almacenan en un Vec. Una vez obtenidos los pagos necesarios
        /// para la verificacion. Se empieza a verificar cada pago, corroborando que:
        /// :> Ninguno de ellos sea un pago pendiente (que tenga una fecha de pago)
        /// :> Que la fecha de pago, sea inferior a la de vencimiento
        /// :> Que el pago no haya sido bonificado
        /// 
        /// Si alguno de estos 3 requisitos no se ha cumplido en algun pago de los necesarios, el método devolverá
        /// "false", indicando que el socio no puede obtener el beneficio, ya que el socio no cumple las condiciones
        /// necesarias. De lo contrario, si cumplió con todos los requisitos en todos los pagos necesarios, el método
        /// devolverá "true", indicando que el socio cumple con las condiciones para obtener el beneficio.
        /// 
        /// Este método recibe una referencia a sí mismo (ClubSemRust), y un tipo de dato u32 que corresponde al DNI de un
        /// socio (dni), y devuelve un tipo de dato bool, que indica si el socio está en condiciones de recibir el
        /// beneficio, o no. 
        fn corroborar_bonificacion(&self, dni: u32) -> bool {

            let mut vec_aux: Vec<Pago> = self.get_pagos_totales(Some(dni));
            //verifico que no sea un vector vacio y que la cantidad de pagos sea la suficiente como para recibir una bonificacion
            if (!vec_aux.is_empty()) && (self.costos.pagos_consecutivos_para_beneficio as usize <= vec_aux.len()) {
                //itero la cantidad de veces necesaria para conseguir la bonificacion
                for _i in 0..self.costos.pagos_consecutivos_para_beneficio {
                    //obtengo el siguiente pago
                    let pago = vec_aux.pop().unwrap();
                    //verifico que haya una fecha de pago, si hay, verifico las condiciones, sino, devuelvo false
                    if let Some (fecha_pago) = pago.fecha_pago {
                        //si la fecha de pago se hizo fuera de termino o el pago esta bonificado, corto la ejecucion y devuelvo false
                        if (fecha_pago > pago.fecha_venci) || (pago.bonificado) {
                            return false;
                        }
                    }
                    else {
                        return false;
                    }
                }
                //si el for termina sin devolver false, entonces se devuelve true
                return true;
            }
            //si el vector es vacio o la cantidad de pagos no alcanza la minima necesaria para la bonificacion, se devuelve false
            return false;
        }

        /// Para esta funcion, primero se comprueba que la variable emision_ultimo_pago contenga algo, de no ser asi, no se han registrado socios
       /// y no tiene sentido disparar el metodo, con lo cual retorna un panick. En el caso de que la emision_ultimo_pago contenga algo, se obtiene
       /// el dia_actual en una variable. Luego se comprueba que el caller sea admin u owner y que el dia actual sea mayor o igual a 30 dias posteriores
       /// a la ultima emision(que nos guardamos previamente en la variable fecha_emision) una vez hechas estas comprobaciones. Para cada socio del vec de socios
       /// se llama a la funncion corroborar_bonificacion, y esta retorna en la variable bonificado si el proximo pago debe ser bonificado o no. Luego se crea una
       /// fecha de vencimiento que sera el dia de hoy + 30 dias y una variable costo cuyo valor se obtiene con la funcion get_costo. Posteriormente
       /// si la variable bonificado previamente dio como resultado true, se le descuenta al costo el beneficio de la variable costos.beneficio
       /// por ultimo,se crea el nnuevo pago con el dni del socio, el costo, la fecha de vencimiento, fecha de pago en None y la variable booleana que
       /// indica si fue bonificado o no. Finalmente se pushea un clone del pago al vec de pagos del club.
       /// Una vez terminado de repetir esto para todos los socios, se actualiza la emision del ultimo pago con la variable previamente creada, dia_actual 
       /// y se retorna true. Si el caller no era admin, staff o la politica estaba desactivada y el dia actual no era mayor o igual a la ultima fecha + 30 dias
       /// entonces el programa arrojara un panick
        #[ink(message)]
        pub fn emitir_pago_mensual (&mut self) -> bool {
            self.emitir_pago_mensual_priv()
        }

        fn emitir_pago_mensual_priv (&mut self) -> bool {
            if let Some (fecha_emision) = self.emision_ultimo_pago {
                let dia_actual = self.get_milisegundos_actuales();
                if self.verificar_permisos() && (dia_actual >= (fecha_emision + 30.from_dias())) { 
                    for socio in &self.socios {
                        let bonificado = self.corroborar_bonificacion(socio.dni);
                        let fecha_venci = self.env().block_timestamp() + 30.from_dias();
                        let mut costo = self.costos.get_costo(&socio.categoria);
                        if bonificado {
                            costo -= self.costos.beneficio;
                        }
                        let nuevo_pago = Pago::new(socio.dni, costo, fecha_venci, None, bonificado);
                        self.pagos.push(nuevo_pago.clone());
                    }
                    self.emision_ultimo_pago = Some (dia_actual);
                    return true;
                }
                else {
                    return panic!("No se cuenta con los permisos necesarios, la politica de autorizacion se encuentra activada, o no han pasado 30 dias desde la ultima emision!");
                } 
            }
            else {
                return panic!("No se han emitido pagos aun!");
            }
        }

        ///La función registrar_socio permite incorporar un nuevo socio al club, recibiendo los datos necesarios : dni, apellido y nombre, id de
        ///la categoría del asociado y, en el caso de que se forme parte de la categoría B, el id del deporte elegido. El método verifica permisos,
        ///por lo que es una acción que usuarios selectos pueden realizar a menos que la política de autorización se encuentre desactivada. La función
        ///rechazará el pedido en caso de que cualquiera de los IDs sean incorrectos, en caso de que el dni ingresado pertenezca a un socio ya
        ///afiliado o de que no se cuenten con los permisos requeridos. Dada una ejecución exitosa, el método retornará la información del
        ///primer pago pendiente del nuevo miembro. 
        #[ink(message)]
        pub fn registrar_socio (&mut self, dni: u32, apellido_y_nombre: String, id_categoria: u32, id_deporte: Option<u32>) -> Option<Pago> {
            return self.registrar_socio_priv(dni, apellido_y_nombre, id_categoria, id_deporte);
        }

        fn registrar_socio_priv (&mut self, dni: u32, apellido_y_nombre: String, id_categoria: u32, id_deporte: Option<u32>) -> Option<Pago> {
            if self.verificar_permisos() && (!self.existe_socio(dni)) {
                let option_categoria = CategoriasSocios::categoria_from_id(&id_categoria);
                match option_categoria {
                    Some (categoria) => {
                        let fecha_hoy = self.env().block_timestamp();
                        let fecha_venci = fecha_hoy + 10.from_dias();
                        match Deporte::deporte_from_id(&id_deporte){
                            Some(deporte) => {let nuevo_socio = Socio::new(apellido_y_nombre.clone(), dni, categoria.clone(), Some(deporte));
                                let pago_pend = Pago::new(dni, self.costos.get_costo(&categoria), fecha_venci, None, false);
                                self.pagos.push(pago_pend.clone());
                                self.socios.push(nuevo_socio.clone());
                                if self.emision_ultimo_pago == None {
                                    self.emision_ultimo_pago = Some (fecha_hoy);
                                    }
                                return Some(pago_pend);    
                                },
                            None=> { let nuevo_socio = Socio::new(apellido_y_nombre.clone(), dni, categoria.clone(), None);
                                let pago_pend = Pago::new(dni, self.costos.get_costo(&categoria), fecha_venci, None, false);
                                self.pagos.push(pago_pend.clone());
                                self.socios.push(nuevo_socio.clone());
                                if self.emision_ultimo_pago == None {
                                    self.emision_ultimo_pago = Some (fecha_hoy);
                                    }
                                return Some(pago_pend);    
                            }
                        }
                    }    
                    None => return panic!("La id de categoria no está entre las presentes!"),
                }
            }
            if (self.existe_socio(dni)){
                return panic!("Ese socio existe!");
            }
            return panic!("No se cuenta con los permisos necesarios o la politica de autorizacion se encuentra activada!");
        }

	    /// Este método se encarga de registrar un pago de un socio.
        /// 
        /// Para comenzar, se verifica que el usuario que llama al método cumpla con los permisos necesarios. De no ser 
        /// así, se arrojará un panic informando lo sucedido. De contar con permisos necesarios, comienza la busqueda
        /// del pago, en el Vec de pagos. Así, se intenta ubicar a un pago que cumpla con las condiciones necesarias, 
        /// que son:
        /// :> Que el pago contenga el DNI del socio.
        /// :> Que el monto ingresado a abonar, sea el mismo que el del pago.
        /// :> Que el pago, no tenga fecha de pago.
        /// 
        /// De no encontrar un pago que cumpla con estas condiciones, se arrojará un panic informando la situación. De lo
        /// contrario, se obtiene el pago del Vec de pagos, y se establece como fecha de pago, la fecha actual, y como
        /// muestra de que se ha registrado el pago, retorna el pago.
        /// 
        /// Este método, recibe una referencia mutable de sí mismo (ClubSemRust), un tipo de dato u32 que hace
        /// referencia al DNI del socio (dni) y un tipo de dato u128, que hace referncia al monto del pago que se quiere
        /// registrar, y como mencionamos anteriormente, devuelve un tipo de dato Pago
        #[ink(message)]
        pub fn realizar_pago (&mut self, dni: u32, monto: u128) -> Pago {
            return self.realizar_pago_priv(dni, monto);
        }

        fn realizar_pago_priv (&mut self, dni: u32, monto: u128) -> Pago {
            if self.verificar_permisos() {

                if self.existe_socio(dni) {
                    let fecha_actual = self.env().block_timestamp();
                    let option_pos = self.pagos.iter().position(|pago| (pago.dni == dni) && (pago.monto == monto) && (pago.fecha_pago == None));
                    if let Some (pos) = option_pos {
                        let pago = self.pagos.get_mut(pos).unwrap();
                        pago.fecha_pago = Some(fecha_actual);
                        return pago.clone();  
                    }

                    return panic!("El cliente no tiene pagos pendientes con el monto ingresado!");
                }
                else {
                    return panic! ("No se ha encontrado un socio con el DNI provisto!");
                }
            }

            return panic!("No se cuenta con los permisos necesarios o la politica de autorizacion se encuentra activada!");
        }
            fn get_milisegundos_actuales(&self)->u64{
                return self.env().block_timestamp();
            }       
    }

    mod tests{
    use super::*;
    //zona de testing

    //Testing de pago
    // Para testear los new, simplemente hago un new de cada pago y en cada test chequeo que la variable del struct pago corresponda con el valor ingresado
    #[ink::test]
    fn pago_new_check_dni_test (){
        let p1 = Pago::new(44933856,2000,1688639827,None,false);
        assert_eq!(p1.dni,44933856);
    }
    #[ink::test]
    fn pago_new_check_monto_test (){
        let p1 = Pago::new(44933856,2000,1688639827,None,false);
        assert_eq!(p1.monto,2000);
    }
    #[ink::test]
    fn pago_new_check_fecha_vencimiento_test (){
        let p1 = Pago::new(44933856,2000,1688639827,None,false);
        assert_eq!(p1.fecha_venci,1688639827);
    }
    #[ink::test]
    fn pago_new_check_fecha_pago_none_test (){
        let p1 = Pago::new(44933856,2000,1688639827,None,false);
        assert_eq!(p1.fecha_pago,None);
    }
    #[ink::test]
    fn pago_new_check_fecha_pago_some_test (){
        let p1 = Pago::new(44933856,2000,1688639827,Some(1688553427),false);
        assert_eq!(p1.fecha_pago,Some(1688553427));
    }
    #[ink::test]
    fn pago_new_check_bonificacion_false_test (){
        let p1 = Pago::new(44933856,2000,1688639827,Some(1688553427),false);
        assert_eq!(p1.bonificado,false);
    }
    #[ink::test]
    fn pago_new_check_bonificacion_true_test (){
        let p1 = Pago::new(44933856,2000,1688639827,Some(1688553427),true);
        assert_eq!(p1.bonificado,true);
    }
    //Para chequear los getters, hago el new de pago, creo una variable x_recibido y compruebo que x_recibido se corresponda con el valor ingresado previamente en el new
    #[ink::test]
    fn get_dni_socio_test(){
        let p1 = Pago::new(44933856,2000,1688639827,Some(1688553427),true);
        let dni_recibido = p1.get_dni_socio();
        assert_eq!(dni_recibido,44933856);
    }
    #[ink::test]
    fn get_monto_test(){
        let p1 = Pago::new(44933856,2000,1688639827,Some(1688553427),true);
        let monto_recibido = p1.get_monto();
        assert_eq!(monto_recibido,2000);
    }
    #[ink::test]
    fn get_fecha_venci_test(){
        let p1 = Pago::new(44933856,2000,1688639827,Some(1688553427),true);
        let fecha_recibida = p1.get_fecha_venci();
        assert_eq!(fecha_recibida,1688639827);
    }
    #[ink::test]
    fn get_fecha_pago_some_test(){
        let p1 = Pago::new(44933856,2000,1688639827,Some(1688553427),true);
        let fecha_recibida = p1.get_fecha_pago();
        assert_eq!(fecha_recibida,Some(1688553427));
    }
    #[ink::test]
    fn get_fecha_pago_none_test(){
        let p1 = Pago::new(44933856,2000,1688639827,None,true);
        let fecha_recibida = p1.get_fecha_pago();
        assert_eq!(fecha_recibida,None);
    }

   ///Para chequear que el set_fecha_pago se ejecute correctamente
   ///creo un nuevo pago, le realizo un set_fecha de pago
   ///y luego utilizo una variable auxiliar fecha_recibida
   ///con el get_fecha_pago (ya testeado previamente)
   ///luego compruebo que fecha_recibida sea igual a lo que
   ///ingrese previamente en el set fecha pago
    #[ink::test]
    fn set_fecha_pago_test(){
        let mut p1 = Pago::new(44933856,2000,1688639827,None,true);
        p1.set_fecha_pago(1688553427);
        let fecha_recibida = p1.get_fecha_pago();
        assert_eq!(fecha_recibida,Some(1688553427));
    }
    //testing de costos categoria

    // para testear los new, simplemente hago un new de cada costo categoria y en cada test chequeo que la variable del struct CostosCategoria corresponda con el valor ingresado
    #[ink::test]
    fn costos_categoria_new_valor_a_test(){
        let c1 = CostosCategoria::new(1000,3000,5000,400,3);
        assert_eq!(c1.a,1000);
    }
    #[ink::test]
    fn costos_categoria_new_valor_b_test(){
        let c1 = CostosCategoria::new(1000,3000,5000,400,3);
        assert_eq!(c1.b,3000);
    }
    #[ink::test]
    fn costos_categoria_new_valor_c_test(){
        let c1 = CostosCategoria::new(1000,3000,5000,400,3);
        assert_eq!(c1.c,5000);
    }
    #[ink::test]
    fn costos_categoria_new_valor_beneficio_test(){
        let c1 = CostosCategoria::new(1000,3000,5000,400,3);
        assert_eq!(c1.beneficio,400);
    }
    #[ink::test]
    fn costos_categoria_new_pagos_consecutivoss_test(){
        let c1 = CostosCategoria::new(1000,3000,5000,400,3);
        assert_eq!(c1.pagos_consecutivos_para_beneficio,3);
    }
    //para chequear los setters, creo una nueva variable CostosCategoria y seteo
    //en diferentes tests sus diferentes variables. Finalmente, compruebo que la variable seteada
    //se corresponda con el nuevo valor seteado
    #[ink::test]
    fn set_costo_a_test(){
        let mut c1 = CostosCategoria::new(1000,3000,5000,400,3);
        c1.set_costo(2500, &CategoriasSocios::A);
        assert_eq!(c1.a,2500);
    }
    #[ink::test]
    fn set_costo_b_test(){
        let mut c1 = CostosCategoria::new(1000,3000,5000,400,3);
        c1.set_costo(2500, &CategoriasSocios::B);
        assert_eq!(c1.b,2500);
    }
    #[ink::test]
    fn set_costo_c_test(){
        let mut c1 = CostosCategoria::new(1000,3000,5000,400,3);
        c1.set_costo(2500, &CategoriasSocios::C);
        assert_eq!(c1.c,2500);
    }
    ///para los getters creo una variable CostosCategoria 
    ///realizo el get que quiero testear y compruebo que se
    ///corresponda con el valor ingresado en el new
    #[ink::test]
    fn get_costo_a_test(){
        let mut c1 = CostosCategoria::new(1000,3000,5000,400,3);
        let costo_recibido = c1.get_costo(&CategoriasSocios::A);
        assert_eq!(costo_recibido,1000);
    }
    #[ink::test]
    fn get_costo_b_test(){
        let mut c1 = CostosCategoria::new(1000,3000,5000,400,3);
        let costo_recibido = c1.get_costo(&CategoriasSocios::B);
        assert_eq!(costo_recibido,3000);
    }
    #[ink::test]
    fn get_costo_c_test(){
        let mut c1 = CostosCategoria::new(1000,3000,5000,400,3);
        let costo_recibido = c1.get_costo(&CategoriasSocios::C);
        assert_eq!(costo_recibido,5000);
    }
    #[ink::test]
    fn set_beneficio_test(){
        let mut c1 = CostosCategoria::new(1000,3000,5000,400,3);
        c1.set_beneficio(500);
        assert_eq!(c1.beneficio,500);
    }
    #[ink::test]
    fn set_pagos_consecutivos_para_beneficio_test(){
        let mut c1 = CostosCategoria::new(1000,3000,5000,400,3);
        c1.set_pagos_consecutivos_para_beneficio(5);
        assert_eq!(c1.pagos_consecutivos_para_beneficio,5);
    }

    //testing de clubSemRust


   /// para testear el new, lo divido en varios tests diferentes para hacerlo lo mas unitario posible
   /// primero creo un club con valores de costos, beneficio y dias para el beneficio, luego entre cada test  
   /// chequeo que se instancien los valores correctamente y que los valores default como el vec de socios,
   /// el vec de pagos o la politica se instancien correctamente  
    #[ink::test]
    fn club_sem_rust_new_costoa_test(){
        let club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let owner = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
        assert_eq!(club1.costos.a,1000);
    }
    #[ink::test]
    fn club_sem_rust_new_costob_test(){
        let club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let owner = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
        assert_eq!(club1.costos.b,2000);
    }
    #[ink::test]
    fn club_sem_rust_new_costoc_test(){
        let club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let owner = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
        assert_eq!(club1.costos.c,5000);
    }
    #[ink::test]
    fn club_sem_rust_new_valor_beneficio_test(){
        let club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let owner = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
        assert_eq!(club1.costos.beneficio,400);
    }
    #[ink::test]
    fn club_sem_rust_new_pagos_consecutivos_beneficio_test(){
        let club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let owner = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
        assert_eq!(club1.costos.pagos_consecutivos_para_beneficio,3);
    }
    #[ink::test]
    fn club_sem_rust_new_emision_ultimo_pago_test(){
        let club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let owner = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
        assert_eq!(club1.emision_ultimo_pago,None);
    }
    #[ink::test]
    fn club_sem_rust_new_owner_test(){
        let club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let caller = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(caller);
        assert_eq!(club1.owner,caller);
    }
    #[ink::test]
    fn club_sem_rust_new_pagos_test(){
        let club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let owner = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
        assert_eq!(club1.pagos.is_empty(),true);
    }
    #[ink::test]
    fn club_sem_rust_new_socios_test(){
        let club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let owner = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
        assert_eq!(club1.socios.is_empty(),true);
    }
    #[ink::test]
    fn club_sem_rust_new_permitidos_test(){
        let club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let owner = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
        assert_eq!(club1.permitidos.is_empty(),true);
    }
    #[ink::test]
    fn club_sem_rust_new_politica_test(){
        let club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let owner = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
        assert_eq!(club1.politica_activada,true);
    }
    //para chequear el get_politica_autorizacion creo un nuevo ClubSemRust (al hacer el new, la politica de autorizacion se inicializa en true)
    //y luego creo una variable esperado en la cual recibo el resultado de get_politica_autorizacion, finalmente, compruebo que esta sea igual a true 
    #[ink::test]
    fn get_politica_autorizacion_owner_call_test(){
        let club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let owner = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
        let esperado=club1.get_politica_autorizacion();
        assert_eq!(esperado,true);
    }
   ///para comprobar que el codigo paniquea si la politica de autorizacion esta activada
   /// y se invoca al metodo get_politica_autorizacion sin ser staff o owner
   /// simplemente creo una variable owner con la cual llamo primero, y realizo el new
   /// y posteriormente le paso el caller a un Not owner (un accountId diferente al que creo la variable club1)
   /// y llamo al metodo get_politica_autorizacion 
    #[ink::test]
    #[should_panic]
    fn get_politica_autorizacion_not_permited_call_test(){
        let club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let owner = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
        let not_an_owner = AccountId::from([0x2;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(not_an_owner);
        let esperado=club1.get_politica_autorizacion();
    }
   /// el quitar_staff devuelve un booleano si se pudo quitar la cuenta correctamente
   /// para chequear que este metodo se comporte de manera correcta, creo un club
   /// agrego al staff un accoundId llamado cuenta1, compruebo que se haya agregado,
   /// chequeando que el len de permitidos sea igual a 1
   /// y luego llamo al metodo quitar_staff finalmente, para asegurarme que 
   /// se elimino, chequeo que el metodo devuelva true y la len de permitidos sea 0
    #[ink::test]
    fn quitar_staff_admin_test(){
        let mut club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let owner = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
        let cuenta1 = AccountId::from([0x2;32]);
        club1.agregar_staff(cuenta1);
        assert_eq!(club1.permitidos.len(),1);
        let ok = club1.quitar_staff(cuenta1);
        assert_eq!(ok,true); 
        assert_eq!(club1.permitidos.len(),0);
    }
    ///para comprobar este panic, creo el ClubSemRust con un accountId
    ///y agrego una persona a staff, luego le paso el caller a otro accountId (not an owner)
    ///e intento llamar a quitar_staff, como no es un owner, panickea
    #[ink::test]
    #[should_panic]
    fn quitar_staff_not_admin_test(){
        let mut club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let owner = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
        let cuenta1 = AccountId::from([0x2;32]);
        club1.agregar_staff(cuenta1);
        let not_an_owner = AccountId::from([0x2;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(not_an_owner);
        let ok = club1.quitar_staff(cuenta1); 
    }
    ///en este test creo una variable accountId (que no se encuentra en staff)
    ///y llamo a quitar_staff con esta variable que cree, al no encontrarse, panickea
    #[ink::test]
    #[should_panic]
    fn quitar_staff_no_encontrado_test(){
        let mut club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let owner = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
        let cuenta1 = AccountId::from([0x2;32]);
        let ok = club1.quitar_staff(cuenta1);
    }

   /// Para chequear el metodo actualizar_costos_categoria, creo un ClubSemRust
   /// y le actualizo el costo una a una a cada categoria en los diferentes tests
   /// al final de ellos, obtengo en una variable nuevo_costo el valor de costos.x
   /// (x es el costos que estoy chequeando) y luego comparo nuevo_costo con el costo
   /// asignado recientemente
    #[ink::test]
    fn actualizar_costos_categoria_a_test(){
        let mut club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let owner = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
        club1.actualizar_costos_categoria(1, 1500);
        let nuevo_costo = club1.costos.a;
        assert_eq!(nuevo_costo,1500);
    }
    #[ink::test]
    fn actualizar_costos_categoria_b_test(){
        let mut club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let owner = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
        club1.actualizar_costos_categoria(2, 1500);
        let nuevo_costo = club1.costos.b;
        assert_eq!(nuevo_costo,1500);
    }
    #[ink::test]
    fn actualizar_costos_categoria_c_test(){
        let mut club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let owner = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
        club1.actualizar_costos_categoria(3, 1500);
        let nuevo_costo = club1.costos.c;
        assert_eq!(nuevo_costo,1500);
    }

    ///en los siguientes casos especiales, chequeo que los panics de una actualizacion de una categoria inexistente
    ///y alguien intentando actualizar una categoria sin tener los permisos necesarios, se disparen correctamente
    #[ink::test]
    #[should_panic]
    fn actualizar_costos_categoria_inexistente_test(){
        let mut club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let owner = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
        club1.actualizar_costos_categoria(5, 1500);
    }
    #[ink::test]
    #[should_panic]
    fn actualizar_costos_categoria_no_tiene_permisos_test(){
        let mut club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let owner = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
        let notowner = AccountId::from([0x2;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(notowner);
        club1.actualizar_costos_categoria(2, 1500);
    }
    ///para chequear el get socio, creo una variable de tipo Socio s1 y luego
    ///creo otra variable de tipo Socio s2 igual a get_socio con el dni que use previamente con s1
    ///finalmente chequeo que s1 y s2 sean iguales para ccomprobar que el get socio funciono correctamente
    #[ink::test]
    fn get_socio_test(){
        let mut club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let owner = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
        let s1 = Socio::new("nico beiser".to_string(), 44956748, CategoriasSocios::C, None);
        club1.registrar_socio(44956748, "nico beiser".to_string(), 3, None);
        let s2 = club1.get_socio(44956748);
        assert_eq!(s1,s2.unwrap());
    }
    ///cuando se hace getsocio con un dni que no existe en la lista de socios, retorna None
    ///por esa razon, realizo el getsocio con un dni inexistente, y compruebo que lo retornado sea igual a None
    #[ink::test]
    fn get_socio_no_existente_test(){
        let mut club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let owner = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
        club1.registrar_socio(44956748, "nico beiser".to_string(), 3, None);
        let socio_no_existente = club1.get_socio(44444);
        assert_eq!(socio_no_existente,None);
    }
   /// get_socio no es una funcion que pueda hacer cualquiera, para testear el siguiente panick
   /// creo una variable clubsemrust, le registro un socio, y luego cambio el caller a un not_owner
   /// el not_owner intenta hacer el get_socio y dispara el panick
    #[ink::test]
    #[should_panic]
    fn get_socio_not_admin_test(){
        let mut club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let owner = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
        let s1 = Socio::new("nico beiser".to_string(), 44956748, CategoriasSocios::C, None);
        club1.registrar_socio(44956748, "nico beiser".to_string(), 3, None);
        let not_owner = AccountId::from([0x2;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(not_owner);
        let s2 = club1.get_socio(44956748);
    }
   ///para este test, realizo lo mismo que en el test previo, con la diferencia que antes de cambiar de caller
   ///cambio la politica de autorizacion, permitiendo que cualquier persona, independientemente de si es socio o no
   ///pueda llamar al get_socio, de esta manera el metodo no genera un panic
    #[ink::test]
    fn get_socio_not_admin_politica_desactivada_test(){
        let mut club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let owner = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
        let s1 = Socio::new("nico beiser".to_string(), 44956748, CategoriasSocios::C, None);
        club1.registrar_socio(44956748, "nico beiser".to_string(), 3, None);
        club1.toggle_politica_autorizacion();
        let not_owner = AccountId::from([0x2;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(not_owner);
        let s2 = club1.get_socio(44956748);
        assert_eq!(s1,s2.unwrap());
    }


   ///para este test, creo una variable clubSemRust y registro un socio.
   ///luego, utilizo el comando set_blocl_timestamp para poner el timestamp en
   ///30 dias posteriores a la emision del ultimo pago (logica entre las lineas 1035 y 1040)
   ///una vez terminado esto, compruebo que el vec de pagos totales del socio que registre sea 1, ya que al registrarlo se crea un nuevo pago pendiente
   ///posteriormente emito el pago mensual y compruebo que el vec de pagos sea 2, y finalmente, realizo el pago y compruebo que el vec siga siendo dos, ya que es pagos totales
   ///y no deberia interferir si esta pago o pendiente
    #[ink::test]
    fn get_pagos_totales_test(){
        let mut club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let owner = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
        club1.registrar_socio(44956748, "nico beiser".to_string(), 3, None);
        let treintadias = 30.from_dias();
        let mut em:u64 = club1.emision_ultimo_pago.unwrap();
        em += treintadias;
        ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(em);
        let vec_esperado1 = club1.get_pagos_totales(Some(44956748));
        assert_eq!(vec_esperado1.len(),1);
        club1.emitir_pago_mensual();
        let vec_esperado2 = club1.get_pagos_totales(Some(44956748));
        assert_eq!(vec_esperado2.len(),2);
        club1.realizar_pago(44956748, 5000);
        let vec_esperado3 = club1.get_pagos_totales(Some(44956748));
        assert_eq!(vec_esperado3.len(),2);
    }
    ///en este test intento llamar al metodo get_pagos_totales luego de cambiar el caller
    ///a un accountId que no es owner ni staff, este test panickea como es de esperar
    #[ink::test]
    #[should_panic]
    fn get_pagos_totales_not_an_owner_test(){
        let mut club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let owner = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
        club1.registrar_socio(44956748, "nico beiser".to_string(), 3, None);
        let not_an_owner = AccountId::from([0x2;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(not_an_owner);
        club1.get_pagos_totales(Some(44956748));
    }
    ///por la logica en la cual esta basada el get_pagos_totales, si el dni no se encuentra en los socios, simplemente retornara un vec vacio
    ///por ello, para testear este caso, compruebo que el vec que retorna al ingresar un dni inexistente en mi vec de socios tiene longitud 0
    #[ink::test]
    fn get_pagos_totales_dni_inexistente_test(){
        let mut club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let owner = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
        club1.registrar_socio(44956748, "nico beiser".to_string(), 3, None);
        let vec_esperado1 = club1.get_pagos_totales(Some(1111111));
        assert_eq!(vec_esperado1.len(),0);
    }
    ///Por la logica en la cual esta basada el get_pagos_totales, si no se ingresa un dni, retornará una copia del 
    ///Vec de pagos del club, así de esta forma, verifico que la longitud del Vec recibido, sea la misma que la del 
    ///Vec de pagos del club.
    #[ink::test]
    fn get_pagos_totales_sin_dni_test(){
        let mut club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let owner = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
        club1.registrar_socio(30320763, "Bouregard Duke".to_string(), 3, None);
        let vec_esperado1 = club1.get_pagos_totales(None);
        assert_eq!(vec_esperado1.len(), club1.pagos.len());
    }

    ///para chequear que no panickee cuando la politica de autorizacion esta desactivada, creo un nuevo club, le agrego un socio
    ///y desactivo la politica, cambio el caller, y llamo al get_pagos_totales. Finalmente compruebo que la informacion 
    ///llegue correctamente esperando un vec de longitud 1
    #[ink::test]
    fn get_pagos_totales_not_an_owner_politica_desactivada_test(){
        let mut club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let owner = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
        club1.registrar_socio(44956748, "nico beiser".to_string(), 3, None);
        club1.toggle_politica_autorizacion();
        let not_an_owner = AccountId::from([0x2;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(not_an_owner);
        let vec_esperado1 = club1.get_pagos_totales(Some(44956748));
        assert_eq!(vec_esperado1.len(),1);
    }

    ///para chequear que el emitir pagos se realice correctamente hay que hacer enfasis en ciertos casos especiales
    ///primero y principal la emision de pago normal, para esto creo un socio (nicolas beiserman) y luego realizo un for de 1..3
    ///emitiendo pagos, luego al salir del for emito otro pago mas (cada vez que emito un pago lo hago avanzando 30 dias, sino el programa
    ///arrojaria un panic) una vez que ya envie el pago despues del for, me traigo en mi variable v1 el vec de pagos totales(con un metodo
    ///ya testeado previamente) y chequeo que este vec tenga una len de 4 (el pago de la creacion, 2 pagos del for y 1 al salir del for)
    ///por otro lado, chequeo el bonificado, aqui dice que cada 3 pagos el 4to debera ser bonificado, por eso primero pago el primer Pago
    ///de registrarme, y realizo un for de 1..3 emitiendo pagos y pagandolos consecutivamente en tiempo y forma, al salir del for
    ///emito un ultimo pago el cual deberia ser con bonificacion de 400 pesos, para chequear esto, creo la variable esperado que contiene
    ///el valor de la categoria elegida - la bonificacion, luego utilizo el previamente mencionado vec de pagos totales y compruebo que
    ///el monto del ultimo pago emitido sea igual a lo que esperaba. Por otra parte tambien testeo que no se le de un bonificado a los que no pagan
    ///para eso cree el socio fran basterrechea, el cual no paga nunca durante todo el test. Luego de testear el ultimo pago de nicolas beiserman
    ///compruebo que estos pagos se le hayan atribuido a la cuenta de fran basterrechea, comprobando su len(debe ser igual a 4) y luego
    ///tomo su ultimo pago y compruebo de la misma manera que hice con nicolas beiserman, el monto a pagar(en este caso sera el monto de su categoria sin descuentos)
    #[ink::test]
    fn emitir_pago_mensual_test(){
        let mut club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let owner = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
        club1.registrar_socio(44987654, "nicolas beiserman".to_string(),3, None);
        club1.registrar_socio(41943654, "fran basterrechea".to_string(),1, None);
        let treintadias = 30.from_dias();
        let mut em:u64 = club1.emision_ultimo_pago.unwrap();
        club1.realizar_pago(44987654, 5000);
        for i in 1..3{
            em += treintadias;
            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(em);
            club1.emitir_pago_mensual();
            club1.realizar_pago(44987654, 5000);
        }
        em += treintadias;
        ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(em);
        club1.emitir_pago_mensual();
        club1.realizar_pago(44987654, 4600);
        let v1 = club1.get_pagos_totales(Some(44987654));
        assert_eq!(v1.len(),4);
        let ultimo_pago = &v1[v1.len()-1];
        let esperado = 5000-400;
        assert_eq!(ultimo_pago.monto,esperado);
        let v1 = club1.get_pagos_totales(Some(41943654));
        assert_eq!(v1.len(),4);
        let ultimo_pago = &v1[v1.len()-1];
        let esperado = 1000;
        assert_eq!(ultimo_pago.monto,esperado);
    }
    #[ink::test]
    #[should_panic]
    fn emitir_pago_mensual_caso_aun_no_se_han_emitido_pagos_test(){
        let mut club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let owner = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
        club1.emitir_pago_mensual();
    }
   ///para este test creo un clubsemrust, registro un socio y hago que solo pasen 10 dias
   ///en el timestamp, con lo cual, luego realizo el emitir_pago_mensual y el programa
   ///panickea ya que aun no han pasado los 30 dias correspondientes
    #[ink::test]
    #[should_panic]
    fn emitir_pago_mensual_caso_aun_no_han_pasado_30_dias_test(){
        let mut club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let owner = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
        club1.registrar_socio(44987654, "nicolas beiserman".to_string(),3, None);
        let diezdias = 10.from_dias();
        let mut em:u64 = club1.emision_ultimo_pago.unwrap();
        em += diezdias;
        ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(em);
        club1.emitir_pago_mensual();
    }
   ///para este test, creo el clubsemrust, registro un socio, hago que pasen 30 dias en
   ///el timestamp y luego cambio el caller a un accoundId diferente del owner
   ///intento emitir el pago mensual y el programa panickea 
    #[ink::test]
    #[should_panic]
    fn emitir_pago_mensual_caso_not_owner_test(){
        let mut club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let owner = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
        club1.registrar_socio(44987654, "nicolas beiserman".to_string(),3, None);
        let treintadias = 30.from_dias();
        let mut em:u64 = club1.emision_ultimo_pago.unwrap();
        em += treintadias;
        ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(em);
        let not_owner = AccountId::from([0x2;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(not_owner);
        club1.emitir_pago_mensual();
    }
   ///para este test se creo el mismo escenario que el caso anterior pero antes de cambiar el
   ///caller, se desactivo la politica de autorizacion, con lo cual el pago se deberia emitir
   ///correctamente. Por ulitmo, para comprobar que el pago se emitio, compruebo la longitud
   ///del vec de pagos, este, deberia tener 2 elementos(el pago del registrar socio y el emitido)
    #[ink::test]
    fn emitir_pago_mensual_caso_not_owner_politica_desactivada_test(){
        let mut club1 = ClubSemRust::new(1000, 2000, 5000, 400, 3);
        let owner = AccountId::from([0x1;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
        club1.registrar_socio(44987654, "nicolas beiserman".to_string(),3, None);
        let treintadias = 30.from_dias();
        let mut em:u64 = club1.emision_ultimo_pago.unwrap();
        em += treintadias;
        ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(em);
        club1.toggle_politica_autorizacion();
        let not_owner = AccountId::from([0x2;32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(not_owner);
        club1.emitir_pago_mensual();
        let v1 = club1.pagos;
        assert_eq!(v1.len(),2);
    }

    //test CategoriasSocios
    #[ink::test]
    fn categoria_from_id_correctas_test() {
        //creo las categroias existentes, y la nula
        let cat_a = CategoriasSocios::categoria_from_id(&1).unwrap();
        let cat_b = CategoriasSocios::categoria_from_id(&2).unwrap();
        let cat_c = CategoriasSocios::categoria_from_id(&3).unwrap();

        //testeo
        assert_eq! (cat_a, CategoriasSocios::A, "Debio ser categoria A");
        assert_eq! (cat_b, CategoriasSocios::B, "Debio ser categoria B");
        assert_eq! (cat_c, CategoriasSocios::C, "Debio ser categoria C");
    }

    #[ink::test]
    #[should_panic]
    fn categoria_from_id_incorrecta_test() {
        //pido una categoria inexistente, esperando el panic
        let cat = CategoriasSocios::categoria_from_id(&4);
    }

    //tests Socio

    #[ink::test]
    fn new_socio_cat_a_y_c_test() {
        //creo los datos de un socio categoria c
        let ape_y_nom_c = "Michael Knight".to_string();
        let dni_c = 35198260;
        let cat_c = CategoriasSocios::categoria_from_id(&3).unwrap();
        let deporte_c = None;

        //creo al socio categoria c
        let socio_c = Socio::new(ape_y_nom_c.clone(), dni_c, cat_c.clone(), deporte_c.clone());

        //creo los datos de un socio categoria a
        let ape_y_nom_a = "Kenneth Hutchinson".to_string();
        let dni_a = 31195032;
        let cat_a = CategoriasSocios::categoria_from_id(&1).unwrap();
        let deporte_a = None;

        //creo al socio categoria a
        let socio_a = Socio::new(ape_y_nom_a.clone(), dni_a, cat_a.clone(), deporte_a.clone());

        //testeo
        assert_eq!(ape_y_nom_c, socio_c.apellido_y_nombre, "Deberia ser el mismo nombre en socio_c");
        assert_eq!(dni_c, socio_c.dni, "Deberia ser el mismo nombre en socio_c");
        assert_eq!(cat_c, socio_c.categoria, "Deberia ser la misma categoria en socio_c");
        assert_eq!(deporte_c, socio_c.deporte, "Deberia ser el mismo deporte en socio_c");

        assert_eq!(ape_y_nom_a, socio_a.apellido_y_nombre, "Deberia ser el mismo nombre en socio_a");
        assert_eq!(dni_a, socio_a.dni, "Deberia ser el mismo nombre en socio_a");
        assert_eq!(cat_a, socio_a.categoria, "Deberia ser la misma categoria en socio_a");
        assert_eq!(deporte_a, socio_a.deporte, "Deberia ser el mismo deporte en socio_a");
    }

    #[ink::test]
    fn new_socio_cat_b_test() {
        //creo los datos de un socio categoria b
        let ape_y_nom = "David Starsky".to_string();
        let dni = 30537974;
        let cat = CategoriasSocios::categoria_from_id(&2).unwrap();
        let deporte = Deporte::deporte_from_id(&Some(2));

        //creo al socio categoria b
        let socio = Socio::new(ape_y_nom.clone(), dni, cat.clone(), deporte.clone());

        //testeo
        assert_eq!(ape_y_nom, socio.apellido_y_nombre, "Deberia ser el mismo nombre en socio");
        assert_eq!(dni, socio.dni, "Deberia ser el mismo nombre en socio");
        assert_eq!(cat, socio.categoria, "Deberia ser la misma categoria en socio");
        assert_eq!(deporte, socio.deporte, "Deberia ser el mismo deporte en socio");
    }

    #[ink::test]
    fn get_apellido_y_nombre_test() {
        //creo un socio
        let ape_y_nom = "David Starsky".to_string();
        let dni = 30537974;
        let cat = CategoriasSocios::categoria_from_id(&2).unwrap();
        let deporte = Deporte::deporte_from_id(&Some(2));
        let socio = Socio::new(ape_y_nom.clone(), dni, cat.clone(), deporte.clone());

        assert_eq!(socio.get_apellido_y_nombre(), ape_y_nom, "Debio ser el mismo nombre");
    }

    #[ink::test]
    fn get_dni_test() {
        //creo un socio
        let ape_y_nom = "David Starsky".to_string();
        let dni = 30537974;
        let cat = CategoriasSocios::categoria_from_id(&2).unwrap();
        let deporte = Deporte::deporte_from_id(&Some(2));
        let socio = Socio::new(ape_y_nom.clone(), dni, cat.clone(), deporte.clone());

        assert_eq!(socio.get_dni(), dni, "Debio ser el mismo DNI");
    }

    #[ink::test]
    fn get_categoria_test() {
        //creo un socio
        let ape_y_nom = "David Starsky".to_string();
        let dni = 30537974;
        let cat = CategoriasSocios::categoria_from_id(&2).unwrap();
        let deporte = Deporte::deporte_from_id(&Some(2));
        let socio = Socio::new(ape_y_nom.clone(), dni, cat.clone(), deporte.clone());

        assert_eq!(socio.get_categoria(), cat, "Debio ser la misma categoria");
    }

    #[ink::test]
    fn get_deporte_cat_test() {
        //creo un socio categoria b
        let ape_y_nom = "David Starsky".to_string();
        let dni = 30537974;
        let cat = CategoriasSocios::categoria_from_id(&2).unwrap();
        let deporte = Deporte::deporte_from_id(&Some(2));
        let socio = Socio::new(ape_y_nom.clone(), dni, cat.clone(), deporte.clone());

        assert_eq!(socio.get_deporte(), deporte, "Debio ser el mismo deporte");
    }

    //tests club

    #[ink::test]
    fn set_owner_correcto_test() {
        //seteo un caller
        let owner = AccountId::from([0x1; 32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);

        //creo un club, y cambio su owner
        let mut club = ClubSemRust::new(5000, 3000, 2000, 1000, 3);
        let nuevo_owner = AccountId::from([0x2; 32]);
        club.set_owner(nuevo_owner);  

        //compruebo
        assert_eq!(club.owner, nuevo_owner, "Debio haber cambiado el owner, ya que el llamado lo realiza el owner");
    }

    #[ink::test]
    #[should_panic]
    fn set_owner_incorrecto_test() {
        //seteo un caller
        let owner = AccountId::from([0x1; 32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);

        //creo un club, cambio al caller, y llamo a set_owner, esperando el panic 
        let mut club = ClubSemRust::new(5000, 3000, 2000, 1000, 3);

        let no_permitido = AccountId::from([0x2; 32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(no_permitido);

        club.set_owner(no_permitido);
    }

    #[ink::test]
    fn agregar_staff_correcto_test() {
        //seteo un caller y creo un club
        let owner = AccountId::from([0x1; 32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);

        let mut club = ClubSemRust::new(5000, 3000, 2000, 1000, 3);

        //agrego un staff y verifico
        assert_eq!(club.agregar_staff(AccountId::from([0x2; 32])), true, "Debio agregar al staff sin problemas");
    }

    #[ink::test]
    #[should_panic]
    fn agregar_staff_existente_test() {
        //seteo un caller y creo un club
        let owner = AccountId::from([0x1; 32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);

        let mut club = ClubSemRust::new(5000, 3000, 2000, 1000, 3);

        //agrego un staff
        let staff = AccountId::from([0x2; 32]);
        club.agregar_staff(staff);

        //intento agregar el mismo, esperando el panic
        club.agregar_staff(staff);
    }

    #[ink::test]
    #[should_panic]
    fn agregar_staff_incorrecto_test() {
        //seteo un caller y creo un club
        let owner = AccountId::from([0x1; 32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);

        let mut club = ClubSemRust::new(5000, 3000, 2000, 1000, 3);

        //cambio el caller, del owner a un posible staff, y llamo a agregar_staff, esperando el panic
        let staff = AccountId::from([0x2; 32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(staff);

        club.agregar_staff(staff);
    }        

    #[ink::test]
    fn es_staff_correcto_test() {
        //seteo un caller y creo un club
        let owner = AccountId::from([0x1; 32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);

        let mut club = ClubSemRust::new(5000, 3000, 2000, 1000, 3);

        //creo y agrego un staff
        let staff = AccountId::from([0x2; 32]);
        club.agregar_staff(staff);

        //seteo el staff agregado como caller, para realizar la comparacion con el metodo es_staff
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(staff);

        assert_eq! (club.es_staff(), true, "Debio ser true, ya que el staff se ha agregado");
    }

    #[ink::test]
    fn es_staff_incorrecto_test() {
        //seteo un caller y creo un club
        let owner = AccountId::from([0x1; 32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);

        let mut club = ClubSemRust::new(5000, 3000, 2000, 1000, 3);

        //agrego un staff
        club.agregar_staff(AccountId::from([0x2; 32]));

        //llamo a es_staff con el owner
        assert_eq! (club.es_staff(), false, "Debio ser false, ya que el caller no es staff");
    }

    #[ink::test]
    fn verificar_permisos_owner_test() {
        //seteo un caller y creo un club
        let owner = AccountId::from([0x1; 32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);

        let mut club = ClubSemRust::new(5000, 3000, 2000, 1000, 3);

        //agrego un staff
        club.agregar_staff(AccountId::from([0x2; 32]));

        assert_eq! (club.verificar_permisos(), true, "Debio ser verdadero");
    }

    #[ink::test]
    fn verificar_permisos_staff_test() {
        //seteo un caller y creo un club
        let owner = AccountId::from([0x1; 32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);

        let mut club = ClubSemRust::new(5000, 3000, 2000, 1000, 3);

        //agrego un staff y lo seteo como caller
        let staff = AccountId::from([0x2; 32]);
        club.agregar_staff(staff);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(staff);

        assert_eq! (club.verificar_permisos(), true, "Debio ser verdadero");
    }

    #[ink::test]
    fn verificar_permisos_no_permitido_con_politica_desactivada_test() {
        //seteo un caller y creo un club
        let owner = AccountId::from([0x1; 32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);

        let mut club = ClubSemRust::new(5000, 3000, 2000, 1000, 3);

        //desactivo la politica
        club.toggle_politica_autorizacion();

        //cambio al owner por un no permitido, y verifico
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(AccountId::from([0x2; 32]));

        assert_eq! (club.verificar_permisos(), true, "Debio ser verdadero, ya que desactive la politica");
    }

    #[ink::test]
    fn verificar_permisos_no_permitido_con_politica_activada_test() {
        //seteo un caller y creo un club
        let owner = AccountId::from([0x1; 32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);

        let club = ClubSemRust::new(5000, 3000, 2000, 1000, 3);

        //cambio al owner por un no permitido, y verifico
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(AccountId::from([0x2; 32]));

        assert_eq! (club.verificar_permisos(), false, "Debio ser falso, ya que la politica está activada");
    }

    #[ink::test]
    fn actualizar_pagos_consecutivos_para_descuento_owner_correcto_test() {
        //seteo un caller y creo un club
        let owner = AccountId::from([0x1; 32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);

        let mut club = ClubSemRust::new(5000, 3000, 2000, 1000, 3);

        //agrego un staff
        club.agregar_staff(AccountId::from([0x2; 32]));

        //llamo a la actualizacion como owner, y verifico
        club.actualizar_pagos_consecutivos_para_descuento(2);

        assert_eq!(club.costos.pagos_consecutivos_para_beneficio, 2, "Debió actualizarse, ya que se llamo al metodo con el owner");
    }

    #[ink::test]
    fn actualizar_pagos_consecutivos_para_descuento_staff_correcto_test() {
        //seteo un caller y creo un club
        let owner = AccountId::from([0x1; 32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);

        let mut club = ClubSemRust::new(5000, 3000, 2000, 1000, 3);

        //agrego un staff
        club.agregar_staff(AccountId::from([0x2; 32]));

        //modifico el caller, para llamar como staff, y verifico
        let staff = AccountId::from([0x2; 32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(staff);

        club.actualizar_pagos_consecutivos_para_descuento(2);
        assert_eq!(club.costos.pagos_consecutivos_para_beneficio, 2, "Debió actualizarse, ya que se llamo al metodo con un staff");
    }

    #[ink::test]
    #[should_panic]
    fn actualizar_pagos_consecutivos_para_descuento_sin_permisos_test() {
        //seteo un caller y creo un club
        let owner = AccountId::from([0x1; 32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);

        let mut club = ClubSemRust::new(5000, 3000, 2000, 1000, 3);

        //cambio el caller por uno no permitido, y realizo el llamado, esperando que paniquee
        let no_permitido = AccountId::from([0x2; 32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(no_permitido);

        club.actualizar_pagos_consecutivos_para_descuento(3);
    }

    #[ink::test]
    #[should_panic]
    fn realizar_pago_llamado_sin_permisos_test() {
        //seteo un caller y creo un club
        let owner = AccountId::from([0x1; 32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);

        let mut club = ClubSemRust::new(5000, 3000, 2000, 1000, 3);

        //cambio el caller por uno no permitido, y realizo el llamado, esperando que paniquee
        let no_permitido = AccountId::from([0x2; 32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(no_permitido);

        club.realizar_pago(30, 5000);
    }

    #[ink::test]
    #[should_panic]
    fn realizar_pago_socio_inexistente_test() {
        //seteo un caller y creo un club
        let owner = AccountId::from([0x1; 32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);

        let mut club = ClubSemRust::new(5000, 3000, 2000, 1000, 3);

        //llamo a realizar un pago de un socio inexiste
        club.realizar_pago(30, 5000);
    }

    #[ink::test]
    fn realizar_pago_correcto_test() {
        //seteo un caller y creo un club
        let owner = AccountId::from([0x1; 32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);

        let mut club = ClubSemRust::new(5000, 3000, 2000, 1000, 3);

        //registro un socio
        let ape_y_nom = "David Starsky".to_string();
        let dni = 30537974;
        club.registrar_socio(dni, ape_y_nom, 2, Some(2));

        //realizo el pago, del pago pendiente recien creado
        let pago = club.realizar_pago(dni, 3000);

        //comparo el pago obetnido, con los datos esperados (menos la fecha de vencimiento)
        assert_eq! (pago.bonificado, false, "No debio ser bonificado, ya que es el primer pago realizado por el socio");
        assert_eq! (pago.dni, dni, "Debio ser el mismo DNI");
        assert_eq! (pago.monto, 3000, "Debio ser el mismo monto");
        assert_ne! (pago.fecha_pago, None, "No debio ser None, ya que deberia contener una fecha de pago");
    }

    #[ink::test]
    #[should_panic]
    fn realizar_pago_sin_permisos_test() {
        //seteo un caller y creo un club
        let owner = AccountId::from([0x1; 32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);

        let mut club = ClubSemRust::new(5000, 3000, 2000, 1000, 3);

        //registro un socio
        let ape_y_nom = "David Starsky".to_string();
        let dni = 30537974;
        club.registrar_socio(dni, ape_y_nom, 2, Some(2));

        //realizo el pago, del pago pendiente recien creado
        club.realizar_pago(dni, 3000);

        //realizo otro pago, esperando el panic, ya que no hay pagos pendientes
        club.realizar_pago(dni, 3000);
    }

    #[ink::test]
    #[should_panic] 
    fn consulta_de_pagos_sin_permisos_test() {
        //seteo un caller y creo un club
        let owner = AccountId::from([0x1; 32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);

        let club = ClubSemRust::new(5000, 3000, 2000, 1000, 3);

        //cambio el caller por un no permitido
        let no_permitido = AccountId::from([0x2; 32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(no_permitido);

        //llamo al metodo esperando el panic
        club.consulta_de_pagos(None);
    }

    #[ink::test]
    #[should_panic]
    fn consulta_de_pagos_dni_no_existente_test() {
        //seteo un caller y creo un club
        let owner = AccountId::from([0x1; 32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);

        let club = ClubSemRust::new(5000, 3000, 2000, 1000, 3);

        //llamo al metodo esperando el panic
        club.consulta_de_pagos(Some(30));
    }

    #[ink::test]
    fn consulta_de_pagos_dni_existente_test() {
        //creo el club "a mano"

        //creo el vec de pagos
        let pago1 = Pago::new(30, 3000, 10, Some(5), false);
        let pago2 = Pago::new(30, 3000, 30, Some(15), false);
        let pago3 = Pago::new(30, 3000, 30, Some(12), false);
        let pago4 = Pago::new(32, 5000, 10, Some(4), false);
        let pago5 = Pago::new(32, 5000, 30, Some(8), false);
        let pago6 = Pago::new(32, 5000, 30, Some(18), false);

        let mut pagos = Vec::new();

        pagos.push(pago1);
        pagos.push(pago2);
        pagos.push(pago3);
        pagos.push(pago4);
        pagos.push(pago5);
        pagos.push(pago6);

        //creo a los socios
        //creo un socio categoria b
        let ape_y_nom_b = "David Starsky".to_string();
        let dni_b = 30;
        let cat_b = CategoriasSocios::categoria_from_id(&2).unwrap();
        let deporte_b = Deporte::deporte_from_id(&Some(2));
        let starsky = Socio::new(ape_y_nom_b.clone(), dni_b, cat_b.clone(), deporte_b.clone());

        //creo un socio categoria a
        let ape_y_nom_a = "Kenneth Hutchinson".to_string();
        let dni_a = 32;
        let cat_a = CategoriasSocios::categoria_from_id(&1).unwrap();
        let deporte_a = None;
        let hutch = Socio::new(ape_y_nom_a.clone(), dni_a, cat_a.clone(), deporte_a.clone());

        //los agrego
        let mut socios = Vec::new();

        socios.push(starsky.clone());
        socios.push(hutch.clone());

        //creo los costos
        let costos = CostosCategoria::new(5000, 3000, 2000, 1000, 3);

        //creo al owner y lo seteo como el caller
        let owner = AccountId::from([0x1; 32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);

        //creo los permitidos
        let permitidos = Vec::new();

        //creo la politica de autorizacion
        let politica_activada = true;

        //creo la ultima emision
        let emision_ultimo_pago = None;

        //creo al club
        let club = ClubSemRust {socios, pagos, costos, owner, permitidos, politica_activada, emision_ultimo_pago};

        //comienzo a crear el vec de tuplas esperado
        //creo el vec de los pagos
        let mut vec_montos1: Vec<u128> = Vec::new();
        for _i in 0..3 {
            vec_montos1.push(3000);
        }

        //creo la tupla esperada
        let tupla1: (Socio, Vec<u128>) = (starsky, vec_montos1);

        //creo el vec de tuplas que quiero obtener
        let mut vec_tuplas_esperado = Vec::new();
        vec_tuplas_esperado.push(tupla1);

        //obtengo la tupla de consulta_de_pagos
        let vec_tuplas_obtenido = club.consulta_de_pagos(Some(dni_b));

        //comparo lo obtenido y esperado
        assert_eq! (vec_tuplas_obtenido, vec_tuplas_esperado, "Debio ser la misma tupla");  
    }

    #[ink::test]
    fn consulta_de_pagos_sin_dni_test() {
        //creo el club "a mano"

        //creo el vec de pagos
        let pago1 = Pago::new(30, 3000, 10, Some(5), false);
        let pago2 = Pago::new(30, 3000, 30, Some(15), false);
        let pago3 = Pago::new(30, 3000, 30, Some(12), false);
        let pago4 = Pago::new(32, 5000, 10, Some(4), false);
        let pago5 = Pago::new(32, 5000, 30, Some(8), false);
        let pago6 = Pago::new(32, 5000, 30, Some(18), false);

        let mut pagos = Vec::new();

        pagos.push(pago1);
        pagos.push(pago2);
        pagos.push(pago3);
        pagos.push(pago4);
        pagos.push(pago5);
        pagos.push(pago6);

        //creo a los socios
        //creo un socio categoria b
        let ape_y_nom_b = "David Starsky".to_string();
        let dni_b = 30;
        let cat_b = CategoriasSocios::categoria_from_id(&2).unwrap();
        let deporte_b = Deporte::deporte_from_id(&Some(2));
        let starsky = Socio::new(ape_y_nom_b.clone(), dni_b, cat_b.clone(), deporte_b.clone());

        //creo un socio categoria a
        let ape_y_nom_a = "Kenneth Hutchinson".to_string();
        let dni_a = 32;
        let cat_a = CategoriasSocios::categoria_from_id(&1).unwrap();
        let deporte_a = None;
        let hutch = Socio::new(ape_y_nom_a.clone(), dni_a, cat_a.clone(), deporte_a.clone());

        //los agrego
        let mut socios = Vec::new();

        socios.push(starsky.clone());
        socios.push(hutch.clone());

        //creo los costos
        let costos = CostosCategoria::new(5000, 3000, 2000, 1000, 3);

        //creo al owner y lo seteo como el caller
        let owner = AccountId::from([0x1; 32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);

        //creo los permitidos
        let permitidos = Vec::new();

        //creo la politica de autorizacion
        let politica_activada = true;

        //creo la ultima emision
        let emision_ultimo_pago = None;

        //creo el club
        let club = ClubSemRust {socios, pagos, costos, owner, permitidos, politica_activada, emision_ultimo_pago};

        //comienzo a crear el vec de tuplas esperado 
        //creo el vec de los pagos de ambos socios
        let mut vec_montos_hutch: Vec<u128> = Vec::new();
        let mut vec_montos_starsky: Vec<u128> = Vec::new();
        for _i in 0..3 {
            vec_montos_starsky.push(3000);
            vec_montos_hutch.push(5000);
        }

        //creo las tupla esperadas
        let tupla1 = (starsky, vec_montos_starsky);
        let tupla2 = (hutch, vec_montos_hutch);

        //creo el vec de tuplas que quiero obtener
        let mut vec_tuplas_esperado = Vec::new();
        vec_tuplas_esperado.push(tupla1);
        vec_tuplas_esperado.push(tupla2);

        //obtengo la tupla de consulta_de_pagos
        let vec_tuplas_obtenido = club.consulta_de_pagos(None);

        //comparo lo obtenido y esperado
        assert_eq! (vec_tuplas_obtenido, vec_tuplas_esperado, "Debieron ser las mismas tuplas");  
    }

    #[ink::test]
    fn corroborar_bonificacion_sin_pagos_suficientes_test() {
        //creo el club "a mano"

        //creo el vec de pagos
        let pago1 = Pago::new(30, 3000, 10, Some(5), false);
        let pago2 = Pago::new(30, 3000, 30, Some(15), false);
        let pago3 = Pago::new(30, 3000, 30, Some(12), false);
        let pago4 = Pago::new(32, 5000, 10, Some(4), false);
        let pago5 = Pago::new(32, 5000, 30, Some(8), false);
        let pago6 = Pago::new(32, 5000, 30, Some(18), false);

        let mut pagos = Vec::new();

        pagos.push(pago1);
        pagos.push(pago2);
        pagos.push(pago3);
        pagos.push(pago4);
        pagos.push(pago5);
        pagos.push(pago6);

        //creo a los socios
        //creo un socio categoria b
        let ape_y_nom_b = "David Starsky".to_string();
        let dni_b = 30;
        let cat_b = CategoriasSocios::categoria_from_id(&2).unwrap();
        let deporte_b = Deporte::deporte_from_id(&Some(2));
        let starsky = Socio::new(ape_y_nom_b.clone(), dni_b, cat_b.clone(), deporte_b.clone());

        //creo un socio categoria a
        let ape_y_nom_a = "Kenneth Hutchinson".to_string();
        let dni_a = 32;
        let cat_a = CategoriasSocios::categoria_from_id(&1).unwrap();
        let deporte_a = None;
        let hutch = Socio::new(ape_y_nom_a.clone(), dni_a, cat_a.clone(), deporte_a.clone());

        //los agrego
        let mut socios = Vec::new();

        socios.push(starsky.clone());
        socios.push(hutch.clone());

        //creo los costos 
        let costos = CostosCategoria::new(5000, 3000, 2000, 1000, 4);

        //creo al owner y lo seteo como el caller
        let owner = AccountId::from([0x1; 32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);

        //creo los permitidos
        let permitidos = Vec::new();

        //creo la politica de autorizacion
        let politica_activada = true;

        //creo la ultima emision
        let emision_ultimo_pago = None;

        //creo el club
        let club = ClubSemRust {socios, pagos, costos, owner, permitidos, politica_activada, emision_ultimo_pago};

        //llamo al metodo de corroborar_bonificacion, sabiendo que la cantidad de pagos necesarios
        //para el beneficio, no alcanza aún

        assert_eq! (club.corroborar_bonificacion(30), false, "No debió otorgar el beneficio");
    }

    #[ink::test]
    fn corroborar_bonificacion_con_pagos_suficientes_test() {
        //creo el club "a mano"

        //creo el vec de pagos
        let pago1 = Pago::new(30, 3000, 10, Some(5), false);
        let pago2 = Pago::new(30, 3000, 30, Some(15), false);
        let pago3 = Pago::new(30, 3000, 30, Some(12), false);
        let pago4 = Pago::new(32, 5000, 10, Some(4), false);
        let pago5 = Pago::new(32, 5000, 30, Some(8), false);
        let pago6 = Pago::new(32, 5000, 30, Some(18), false);

        let mut pagos = Vec::new();

        pagos.push(pago1);
        pagos.push(pago2);
        pagos.push(pago3);
        pagos.push(pago4);
        pagos.push(pago5);
        pagos.push(pago6);

        //creo a los socios
        //creo un socio categoria b
        let ape_y_nom_b = "David Starsky".to_string();
        let dni_b = 30;
        let cat_b = CategoriasSocios::categoria_from_id(&2).unwrap();
        let deporte_b = Deporte::deporte_from_id(&Some(2));
        let starsky = Socio::new(ape_y_nom_b.clone(), dni_b, cat_b.clone(), deporte_b.clone());

        //creo un socio categoria a
        let ape_y_nom_a = "Kenneth Hutchinson".to_string();
        let dni_a = 32;
        let cat_a = CategoriasSocios::categoria_from_id(&1).unwrap();
        let deporte_a = None;
        let hutch = Socio::new(ape_y_nom_a.clone(), dni_a, cat_a.clone(), deporte_a.clone());

        //los agrego
        let mut socios = Vec::new();

        socios.push(starsky.clone());
        socios.push(hutch.clone());

        //creo los costos 
        let costos = CostosCategoria::new(5000, 3000, 2000, 1000, 2);

        //creo al owner y lo seteo como el caller
        let owner = AccountId::from([0x1; 32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);

        //creo los permitidos
        let permitidos = Vec::new();

        //creo la politica de autorizacion
        let politica_activada = true;

        //creo la ultima emision
        let emision_ultimo_pago = None;

        //creo el club
        let club = ClubSemRust {socios, pagos, costos, owner, permitidos, politica_activada, emision_ultimo_pago};

        //llamo al metodo de corroborar_bonificacion, sabiendo que la cantidad de pagos necesarios
        //para el beneficio, alcanza

        assert_eq! (club.corroborar_bonificacion(30), true, "Debió otorgar el beneficio");
    }

    #[ink::test]
    fn corroborar_bonificacion_con_pago_pendiente_test() {
        //creo el club "a mano"

        //creo el vec de pagos
        let pago1 = Pago::new(30, 3000, 10, Some(5), false);
        let pago2 = Pago::new(30, 3000, 30, None, false);
        let pago3 = Pago::new(30, 3000, 30, Some(12), false);
        let pago4 = Pago::new(32, 5000, 10, Some(4), false);
        let pago5 = Pago::new(32, 5000, 30, Some(8), false);
        let pago6 = Pago::new(32, 5000, 30, Some(18), false);

        let mut pagos = Vec::new();

        pagos.push(pago1);
        pagos.push(pago2);
        pagos.push(pago3);
        pagos.push(pago4);
        pagos.push(pago5);
        pagos.push(pago6);

        //creo a los socios
        //creo un socio categoria b
        let ape_y_nom_b = "David Starsky".to_string();
        let dni_b = 30;
        let cat_b = CategoriasSocios::categoria_from_id(&2).unwrap();
        let deporte_b = Deporte::deporte_from_id(&Some(2));
        let starsky = Socio::new(ape_y_nom_b.clone(), dni_b, cat_b.clone(), deporte_b.clone());

        //creo un socio categoria a
        let ape_y_nom_a = "Kenneth Hutchinson".to_string();
        let dni_a = 32;
        let cat_a = CategoriasSocios::categoria_from_id(&1).unwrap();
        let deporte_a = None;
        let hutch = Socio::new(ape_y_nom_a.clone(), dni_a, cat_a.clone(), deporte_a.clone());

        //los agrego
        let mut socios = Vec::new();

        socios.push(starsky.clone());
        socios.push(hutch.clone());

        //creo los costos 
        let costos = CostosCategoria::new(5000, 3000, 2000, 1000, 2);

        //creo al owner y lo seteo como el caller
        let owner = AccountId::from([0x1; 32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);

        //creo los permitidos
        let permitidos = Vec::new();

        //creo la politica de autorizacion
        let politica_activada = true;

        //creo la ultima emision
        let emision_ultimo_pago = None;

        //creo el club
        let club = ClubSemRust {socios, pagos, costos, owner, permitidos, politica_activada, emision_ultimo_pago};

        //llamo al metodo de corroborar_bonificacion, sabiendo que la cantidad de pagos necesarios
        //para el beneficio, alcanza

        assert_eq! (club.corroborar_bonificacion(30), false, "No debió otorgar el beneficio");
    }

    #[ink::test]
    fn corroborar_bonificacion_con_pago_vencido_test() {
        //creo el club "a mano"

        //creo el vec de pagos
        let pago1 = Pago::new(30, 3000, 10, Some(5), false);
        let pago2 = Pago::new(30, 3000, 30, Some(31), false);
        let pago3 = Pago::new(30, 3000, 30, Some(12), false);
        let pago4 = Pago::new(32, 5000, 10, Some(4), false);
        let pago5 = Pago::new(32, 5000, 30, Some(8), false);
        let pago6 = Pago::new(32, 5000, 30, Some(18), false);

        let mut pagos = Vec::new();

        pagos.push(pago1);
        pagos.push(pago2);
        pagos.push(pago3);
        pagos.push(pago4);
        pagos.push(pago5);
        pagos.push(pago6);

        //creo a los socios
        //creo un socio categoria b
        let ape_y_nom_b = "David Starsky".to_string();
        let dni_b = 30;
        let cat_b = CategoriasSocios::categoria_from_id(&2).unwrap();
        let deporte_b = Deporte::deporte_from_id(&Some(2));
        let starsky = Socio::new(ape_y_nom_b.clone(), dni_b, cat_b.clone(), deporte_b.clone());

        //creo un socio categoria a
        let ape_y_nom_a = "Kenneth Hutchinson".to_string();
        let dni_a = 32;
        let cat_a = CategoriasSocios::categoria_from_id(&1).unwrap();
        let deporte_a = None;
        let hutch = Socio::new(ape_y_nom_a.clone(), dni_a, cat_a.clone(), deporte_a.clone());

        //los agrego
        let mut socios = Vec::new();

        socios.push(starsky.clone());
        socios.push(hutch.clone());

        //creo los costos 
        let costos = CostosCategoria::new(5000, 3000, 2000, 1000, 2);

        //creo al owner y lo seteo como el caller
        let owner = AccountId::from([0x1; 32]);
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);

        //creo los permitidos
        let permitidos = Vec::new();

        //creo la politica de autorizacion
        let politica_activada = true;

        //creo la ultima emision
        let emision_ultimo_pago = None;

        //creo el club
        let club = ClubSemRust {socios, pagos, costos, owner, permitidos, politica_activada, emision_ultimo_pago};

        //llamo al metodo de corroborar_bonificacion, sabiendo que la cantidad de pagos necesarios
        //para el beneficio, alcanza

        assert_eq! (club.corroborar_bonificacion(30), false, "No debió otorgar el beneficio");
    }

    
   // test get_socios
   #[ink::test]
   fn get_socios_test(){
       let s1 = Socio{apellido_y_nombre : "Giordano Luca".to_string(), dni : 222, categoria : CategoriasSocios::A, deporte : None};
       let s2 = Socio{apellido_y_nombre : "Basterrechea Franco".to_string(), dni : 333, categoria : CategoriasSocios::B, deporte : Some(Deporte::Futbol)};
       let s3 = Socio{apellido_y_nombre : "Beiserman Nicolas".to_string(), dni : 444, categoria : CategoriasSocios::A, deporte : None};
       let mut socios : Vec<Socio> = Vec::new();
       socios.push(s1);
       socios.push(s2);
       socios.push(s3);
       let costos = CostosCategoria::new(5000, 2000, 1000, 500, 3);
       let pagos : Vec<Pago>= Vec::new();
       let owner = AccountId::from([0x1; 32]);
       ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
       let permitidos : Vec<AccountId> = Vec::new();
       let politica_activada = true;
       let emision_ultimo_pago = None;
       let club = ClubSemRust{costos, socios:socios.clone(), pagos, owner, permitidos, politica_activada, emision_ultimo_pago};
       let socios_devuelto = club.get_socios();
       assert_eq!(socios_devuelto.clone(), socios.clone(), "Ocurrió un error al esperar el vector de socios {:#?}, se recibió {:#?}.", socios, socios_devuelto);
   }
   #[ink::test]
   fn get_socios_vacio_test(){
       let mut socios : Vec<Socio> = Vec::new();
       let costos = CostosCategoria::new(5000, 2000, 1000, 500, 3);
       let pagos : Vec<Pago>= Vec::new();
       let owner = AccountId::from([0x1; 32]);
       ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
       let permitidos : Vec<AccountId> = Vec::new();
       let politica_activada = true;
       let emision_ultimo_pago = None;
       let club = ClubSemRust{costos, socios:socios.clone(), pagos, owner, permitidos, politica_activada, emision_ultimo_pago};
       let socios_devuelto = club.get_socios();
       assert_eq!(socios_devuelto.clone(), socios.clone(), "Ocurrió un error al esperar el vector de socios vacio! Se recibió {:#?}", socios_devuelto);
   }
   #[ink::test]
   #[should_panic]
   fn get_socios_sin_permisos_test(){
       let mut socios : Vec<Socio> = Vec::new();
       let costos = CostosCategoria::new(5000, 2000, 1000, 500, 3);
       let pagos : Vec<Pago>= Vec::new();
       let owner = AccountId::from([0x1; 32]);
       let not_owner = AccountId::from([0x2; 32]);
       ink::env::test::set_caller::<ink::env::DefaultEnvironment>(not_owner);
       let permitidos : Vec<AccountId> = Vec::new();
       let politica_activada = true;
       let emision_ultimo_pago = None;
       let club = ClubSemRust{costos, socios, pagos, owner, permitidos, politica_activada, emision_ultimo_pago};
       club.get_socios();
   }

   // test toggle_politica_autorizacion
   #[ink::test]
   fn toggle_politica_autorizacion_test(){
       let mut socios : Vec<Socio> = Vec::new();
       let costos = CostosCategoria::new(5000, 2000, 1000, 500, 3);
       let pagos : Vec<Pago>= Vec::new();
       let owner = AccountId::from([0x1; 32]);
       ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
       let permitidos : Vec<AccountId> = Vec::new();
       let politica_activada = true;
       let emision_ultimo_pago = None;
       let mut club = ClubSemRust{costos, socios, pagos, owner, permitidos, politica_activada, emision_ultimo_pago};
       let valor_politica = club.get_politica_autorizacion();
       assert_eq!(club.toggle_politica_autorizacion(), !valor_politica, "La función no cumplió su cometido!");
   }
   #[ink::test]
   #[should_panic]
   fn toggle_politica_autorizacion_sin_permisos_test(){
       let mut socios : Vec<Socio> = Vec::new();
       let costos = CostosCategoria::new(5000, 2000, 1000, 500, 3);
       let pagos : Vec<Pago>= Vec::new();
       let owner = AccountId::from([0x1; 32]);
       let not_owner = AccountId::from([0x2; 32]);
       ink::env::test::set_caller::<ink::env::DefaultEnvironment>(not_owner);
       let permitidos : Vec<AccountId> = Vec::new();
       let politica_activada = true;
       let emision_ultimo_pago = None;
       let mut club = ClubSemRust{costos, socios, pagos, owner, permitidos, politica_activada, emision_ultimo_pago};
       club.toggle_politica_autorizacion();
   }

   // test es_admin
   #[ink::test]
   fn es_admin_es_test(){
       let mut socios : Vec<Socio> = Vec::new();
       let costos = CostosCategoria::new(5000, 2000, 1000, 500, 3);
       let pagos : Vec<Pago>= Vec::new();
       let owner = AccountId::from([0x1; 32]);
       ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
       let permitidos : Vec<AccountId> = Vec::new();
       let politica_activada = true;
       let emision_ultimo_pago = None;
       let club = ClubSemRust{costos, socios, pagos, owner, permitidos, politica_activada, emision_ultimo_pago};
       assert_eq!(club.es_admin(), true, "El caller no es admin!");
   }
   #[ink::test]
   fn es_admin_no_es_test(){
       let mut socios : Vec<Socio> = Vec::new();
       let costos = CostosCategoria::new(5000, 2000, 1000, 500, 3);
       let pagos : Vec<Pago>= Vec::new();
       let owner = AccountId::from([0x1; 32]);
       let not_owner = AccountId::from([0x2; 32]);
       ink::env::test::set_caller::<ink::env::DefaultEnvironment>(not_owner);
       let permitidos : Vec<AccountId> = Vec::new();
       let politica_activada = true;
       let emision_ultimo_pago = None;
       let club = ClubSemRust{costos, socios, pagos, owner, permitidos, politica_activada, emision_ultimo_pago};
       assert_eq!(club.es_admin(), false, "El caller es admin!");
   }

   // test actualizar_costo_beneficio
   #[ink::test]
   fn actualizar_costo_beneficio_test(){
       let mut socios : Vec<Socio> = Vec::new();
       let costos = CostosCategoria::new(5000, 2000, 1000, 500, 3);
       let pagos : Vec<Pago>= Vec::new();
       let owner = AccountId::from([0x1; 32]);
       ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
       let permitidos : Vec<AccountId> = Vec::new();
       let politica_activada = true;
       let emision_ultimo_pago = None;
       let mut club = ClubSemRust{costos, socios, pagos, owner, permitidos, politica_activada, emision_ultimo_pago};
       let nuevo_costo = 300;
       assert_eq!(club.actualizar_costo_beneficio(nuevo_costo), true, "No se actualizó!")
   }
   #[ink::test]
   #[should_panic]
   fn actualizar_costo_beneficio_sin_permisos_test(){
       let mut socios : Vec<Socio> = Vec::new();
       let costos = CostosCategoria::new(5000, 2000, 1000, 500, 3);
       let pagos : Vec<Pago>= Vec::new();
       let owner = AccountId::from([0x1; 32]);
       let not_owner = AccountId::from([0x2; 32]);
       ink::env::test::set_caller::<ink::env::DefaultEnvironment>(not_owner);
       let permitidos : Vec<AccountId> = Vec::new();
       let politica_activada = true;
       let emision_ultimo_pago = None;
       let mut club = ClubSemRust{costos, socios, pagos, owner, permitidos, politica_activada, emision_ultimo_pago};
       let nuevo_costo = 300;
       assert_eq!(club.actualizar_costo_beneficio(nuevo_costo), false, "Se actualizó el costo cuando no deberia!")
   }

   // test existe_socio
   #[ink::test]
   fn existe_socio_test(){
       let s1 = Socio{apellido_y_nombre : "Giordano Luca".to_string(), dni : 222, categoria : CategoriasSocios::A, deporte : None};
       let s2 = Socio{apellido_y_nombre : "Basterrechea Franco".to_string(), dni : 333, categoria : CategoriasSocios::B, deporte : Some(Deporte::Futbol)};
       let s3 = Socio{apellido_y_nombre : "Beiserman Nicolas".to_string(), dni : 444, categoria : CategoriasSocios::A, deporte : None};
       let mut socios : Vec<Socio> = Vec::new();
       socios.push(s1);
       socios.push(s2);
       socios.push(s3);
       let costos = CostosCategoria::new(5000, 2000, 1000, 500, 3);
       let pagos : Vec<Pago>= Vec::new();
       let owner = AccountId::from([0x1; 32]);
       ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
       let permitidos : Vec<AccountId> = Vec::new();
       let politica_activada = true;
       let emision_ultimo_pago = None;
       let club = ClubSemRust{costos, socios, pagos, owner, permitidos, politica_activada, emision_ultimo_pago};
       assert_eq!(club.existe_socio(222), true, "No se encontró el socio cuando el socio si existe!");
   }
   #[ink::test]
   fn existe_socio_no_existe_test(){
       let s1 = Socio{apellido_y_nombre : "Giordano Luca".to_string(), dni : 222, categoria : CategoriasSocios::A, deporte : None};
       let s2 = Socio{apellido_y_nombre : "Basterrechea Franco".to_string(), dni : 333, categoria : CategoriasSocios::B, deporte : Some(Deporte::Futbol)};
       let s3 = Socio{apellido_y_nombre : "Beiserman Nicolas".to_string(), dni : 444, categoria : CategoriasSocios::A, deporte : None};
       let mut socios : Vec<Socio> = Vec::new();
       socios.push(s1);
       socios.push(s2);
       socios.push(s3);
       let costos = CostosCategoria::new(5000, 2000, 1000, 500, 3);
       let pagos : Vec<Pago>= Vec::new();
       let owner = AccountId::from([0x1; 32]);
       ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
       let permitidos : Vec<AccountId> = Vec::new();
       let politica_activada = true;
       let emision_ultimo_pago = None;
       let club = ClubSemRust{costos, socios, pagos, owner, permitidos, politica_activada, emision_ultimo_pago};
       assert_eq!(club.existe_socio(7777), false, "Se encontró el socio cuando el socio no existe!");
   }
   
   // test registrar_socio
   #[ink::test]
   #[should_panic]
   fn registrar_socio_ya_existe_test(){
       let s1 = Socio{apellido_y_nombre : "Giordano Luca".to_string(), dni : 222, categoria : CategoriasSocios::A, deporte : None};
       let s2 = Socio{apellido_y_nombre : "Basterrechea Franco".to_string(), dni : 333, categoria : CategoriasSocios::B, deporte : Some(Deporte::Futbol)};
       let s3 = Socio{apellido_y_nombre : "Beiserman Nicolas".to_string(), dni : 444, categoria : CategoriasSocios::A, deporte : None};
       let mut socios : Vec<Socio> = Vec::new();
       socios.push(s1);
       socios.push(s2);
       socios.push(s3);
       let costos = CostosCategoria::new(5000, 2000, 1000, 500, 3);
       let pagos : Vec<Pago>= Vec::new();
       let owner = AccountId::from([0x1; 32]);
       ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
       let permitidos : Vec<AccountId> = Vec::new();
       let politica_activada = true;
       let emision_ultimo_pago = None;
       let mut club = ClubSemRust{costos, socios, pagos, owner, permitidos, politica_activada, emision_ultimo_pago};
       let resultado = club.registrar_socio(222, "Giordano Luca".to_string(), 2, Some(3));
   }
   #[ink::test]
   #[should_panic]
   fn registrar_socio_ya_existe_deporte_incorrecto_test(){
       let s1 = Socio{apellido_y_nombre : "Giordano Luca".to_string(), dni : 222, categoria : CategoriasSocios::A, deporte : None};
       let s2 = Socio{apellido_y_nombre : "Basterrechea Franco".to_string(), dni : 333, categoria : CategoriasSocios::B, deporte : Some(Deporte::Futbol)};
       let s3 = Socio{apellido_y_nombre : "Beiserman Nicolas".to_string(), dni : 444, categoria : CategoriasSocios::A, deporte : None};
       let mut socios : Vec<Socio> = Vec::new();
       socios.push(s1);
       socios.push(s2);
       socios.push(s3);
       let costos = CostosCategoria::new(5000, 2000, 1000, 500, 3);
       let pagos : Vec<Pago>= Vec::new();
       let owner = AccountId::from([0x1; 32]);
       ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
       let permitidos : Vec<AccountId> = Vec::new();
       let politica_activada = true;
       let emision_ultimo_pago = None;
       let mut club = ClubSemRust{costos, socios, pagos, owner, permitidos, politica_activada, emision_ultimo_pago};
       let resultado = club.registrar_socio(555, "Cerati Gustavo".to_string(), 2, Some(11));
   }
   #[ink::test]
   #[should_panic]
   fn registrar_socio_ya_existe_categoria_incorrecta_test(){
       let s1 = Socio{apellido_y_nombre : "Giordano Luca".to_string(), dni : 222, categoria : CategoriasSocios::A, deporte : None};
       let s2 = Socio{apellido_y_nombre : "Basterrechea Franco".to_string(), dni : 333, categoria : CategoriasSocios::B, deporte : Some(Deporte::Futbol)};
       let s3 = Socio{apellido_y_nombre : "Beiserman Nicolas".to_string(), dni : 444, categoria : CategoriasSocios::A, deporte : None};
       let mut socios : Vec<Socio> = Vec::new();
       socios.push(s1);
       socios.push(s2);
       socios.push(s3);
       let costos = CostosCategoria::new(5000, 2000, 1000, 500, 3);
       let pagos : Vec<Pago>= Vec::new();
       let owner = AccountId::from([0x1; 32]);
       ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
       let permitidos : Vec<AccountId> = Vec::new();
       let politica_activada = true;
       let emision_ultimo_pago = None;
       let mut club = ClubSemRust{costos, socios, pagos, owner, permitidos, politica_activada, emision_ultimo_pago};
       let resultado = club.registrar_socio(555, "Cerati Gustavo".to_string(), 45, Some(3));
   }
   #[ink::test]
   #[should_panic]
   fn registrar_socio_sin_permisos_test() {
    let s1 = Socio{apellido_y_nombre : "David Starsky".to_string(), dni : 537, categoria : CategoriasSocios::B, deporte : Some(Deporte::Basquet)};
    let s2 = Socio{apellido_y_nombre : "Kenneth Hutchinson".to_string(), dni : 540, categoria : CategoriasSocios::B, deporte : Some(Deporte::Basquet)};
    let s3 = Socio{apellido_y_nombre : "Huggy Bear".to_string(), dni : 440, categoria : CategoriasSocios::A, deporte : None};
    let mut socios : Vec<Socio> = Vec::new();
    socios.push(s1);
    socios.push(s2);
    socios.push(s3);
    let costos = CostosCategoria::new(5000, 2000, 1000, 500, 3);
    let pagos : Vec<Pago>= Vec::new();
    let owner = AccountId::from([0x1; 32]);
    ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
    let permitidos : Vec<AccountId> = Vec::new();
    let politica_activada = true;
    let emision_ultimo_pago = None;
    let mut club = ClubSemRust{costos, socios, pagos, owner, permitidos, politica_activada, emision_ultimo_pago}; 
    let no_permitido = AccountId::from([0x2; 32]);
    ink::env::test::set_caller::<ink::env::DefaultEnvironment>(no_permitido);
    let resultado = club.registrar_socio(982, "Michael Knight".to_string(), 1, None);
   }
   //test ConversionFechas
   #[ink::test]
   fn conversion_fechas_test(){
       assert_eq!(0.from_segundos(), 0);
       assert_eq!(1.from_segundos(), 1000);
       assert_eq!(10.from_segundos(), 10000);
       assert_eq!(0.from_horas(), 0);
       assert_eq!(1.from_horas(), 3600000);
       assert_eq!(10.from_horas(), 36000000);
       assert_eq!(0.from_dias(), 0);
       assert_eq!(1.from_dias(), 86400000);
       assert_eq!(10.from_dias(), 864000000);
       assert_eq!(0.from_semanas(), 0);
       assert_eq!(1.from_semanas(), 604800000);
       assert_eq!(10.from_semanas(), 6048000000);
       assert_eq!(0.from_meses(), 0);
       assert_eq!(1.from_meses(), 2629743000);
       assert_eq!(10.from_meses(), 26297430000);
       assert_eq!(0.from_anios(), 0);
       assert_eq!(1.from_anios(), 31556926000);
       assert_eq!(10.from_anios(), 315569260000);
   }
   //tests deporte
   #[ink::test]
   fn deporte_from_id_test(){
       assert_eq!(Deporte::deporte_from_id(&Some(1)), Some(Deporte::Futbol));
       assert_eq!(Deporte::deporte_from_id(&Some(2)), Some(Deporte::Basquet));
       assert_eq!(Deporte::deporte_from_id(&Some(3)), Some(Deporte::Rugby));
       assert_eq!(Deporte::deporte_from_id(&Some(4)), Some(Deporte::Hockey));
       assert_eq!(Deporte::deporte_from_id(&Some(5)), Some(Deporte::Natacion));
       assert_eq!(Deporte::deporte_from_id(&Some(6)), Some(Deporte::Tenis));
       assert_eq!(Deporte::deporte_from_id(&Some(7)), Some(Deporte::Paddle));
       assert_eq!(Deporte::deporte_from_id(&Some(8)), Some(Deporte::Gimnasio));
       assert_eq!(Deporte::deporte_from_id(&None), None);
   }

   #[ink::test]
   #[should_panic]
   fn deporte_from_id_incorrecta_test(){
       let dep = Deporte::deporte_from_id(&Some(10));
   }
    }
}