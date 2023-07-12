#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod reportes_club_sem_rust {

    use ClubSemRust::ClubSemRust::Socio;
    use ClubSemRust::ClubSemRust::Deporte;
    use ClubSemRust::ClubSemRust::CategoriasSocios;
    use ClubSemRust::ClubSemRust::Pago;
    use ink::prelude::vec::Vec;
    use ClubSemRust::ClubSemRustRef;

    /// Este struct, nos permite "mockear" infromación para realizar los tests. Es su único fin. ya que para la logica del
    /// contrato, no cumple con ninguna función, solo la de testear el mismo. Contiene: un tipo de dato Vec que almacena
    /// socios (socios), un tipo de dato Vec que almacena pagos (pagos), y un tipo de dato bool (permisos), que indica 
    /// si el contrato de Reportes tiene permisos para operar sobre los metodos del Club.
    #[derive(scale::Decode, scale::Encode, Debug, Clone, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct ClubTesting{
        permisos: bool,
        socios: Vec<Socio>,
        pagos: Vec<Pago>,
    }

    /// Este trait se ha creado con la finalidad de poder "mockear" data proveniente del Club, y así poder realizar los 
    /// tests de la funcionalidad de los metodos del contrato Reportes.
    /// 
    /// De esta forma, contiene 3 metodos abstractos: verificar_permiso_trait, get_socios_trait, y get_pagos_totales_trait.
    /// 
    /// Así, el ClubTesting lo implementa de manera que clona los datos que contiene. Mientras, el ReportesClubSemRust lo
    /// implementa de manera que obtiene los datos pedidos desde el Club, obteniendo data veraz, o verdadera.
    /// 
    /// Cabe aclarar, que la implementación del struct ClubTesting, solo se ejecutará en casos de testing, mientras que
    /// la implementacion para ReportesClubSemRust, se ejecutará siempre que este fuera de una ejecución des testing.
    trait ClubSemRustTrait{
        fn verificar_permisos_trait(&self) -> bool;
        fn get_socios_trait(&self) -> Vec<Socio>;
        fn get_pagos_totales_trait(&self) -> Vec<Pago>;
    }

    #[cfg(test)]
    impl ClubSemRustTrait for ClubTesting{

        /// Este método, hace una copia del contenido del campo "permisos" del struct ClubTesting y la retorna. Así,
        /// devuelve un tipo de dato bool.
        fn verificar_permisos_trait(&self) -> bool{
            return self.permisos;
        }

        /// Este método, clona el contenido del campo "socios" del struct ClubTesting y la retorna. Así,
        /// devuelve un tipo de dato Vec que almacena socios.
        fn get_socios_trait(&self) -> Vec<Socio>{
            return self.socios.clone();
        }

        /// Este método, clona el contenido del campo "pagos" del struct ClubTesting y la retorna. Así,
        /// devuelve un tipo de dato Vec que almacena pagos.
        fn get_pagos_totales_trait(&self) -> Vec<Pago>{
            return self.pagos.clone();
        }
    } 

    #[cfg(not(test))]
    impl ClubSemRustTrait for ClubSemRustRef{

        /// Este método, hace un llamado al método "verificar_permisos" del Club, y devuelve el resultado de este llamado.
        fn verificar_permisos_trait(&self) -> bool{
            return self.verificar_permisos();
        }

        /// Este método, hace un llamado al método "get_socios" del Club, y devuelve el resultado de este llamado.
        fn get_socios_trait(&self) -> Vec<Socio>{
            return self.get_socios();
        }

        /// Este método, hace un llamado al método "get_pagos_totales" del Club, y devuelve el resultado de este llamado.
        fn get_pagos_totales_trait(&self) -> Vec<Pago>{
            return self.get_pagos_totales(None);
        }
    }
    /// Este struct, contiene una referencia al Contrato sobre el cual realiza los reportes, en este caso, al Club, 
    /// siendo así, la referencia a este contrato, su único tipo de dato.
    #[ink(storage)]
    pub struct ReportesClubSemRust {
        #[cfg(not(test))]
        club :  ClubSemRustRef,
        #[cfg(test)]
        club : ClubTesting,
    }
    impl ReportesClubSemRust {
        /// Este método, es el constructor del contrato. Recibe una referencia al Club el cual contiene los datos sobre
        /// el cual se quieren realizar los reportes (club), y crea una instancia del contrato ReportesClubSemRust, y la
        /// devuelve.
        #[cfg(not(test))]
        #[ink(constructor)]
        pub fn new(club : ClubSemRustRef) -> Self {
            return ReportesClubSemRust::new_priv(club);
        }
        
        #[cfg(not(test))]
        fn new_priv (club : ClubSemRustRef) -> Self {
            return Self {club};
        }

        ///Este método, es el constructor del contrato en los casos de testing
        #[cfg(test)]
        pub fn new(club : ClubTesting) -> Self{
            return Self{club};
        }
        /// Este método, se encarga de realizar un listado con los socios morosos del club, los cuales son aquellos que 
        /// tienen, al menos, un pago pendiente vencido, es decir, con una fecha de vencimiento que es menor a la fecha 
        /// en la que se realiza este listado.
        /// 
        /// De esta forma, se verifica que este contrato, contenga los permisos en el Club para realizar el reporte.
        /// De no contar con estos, arroja un panic informando lo sucedido. De lo contrario, se comienza a buscar los
        /// socios morosos. Así, se obtiene una marca de la fecha actual y se crea la estructura que almacenará a los
        /// socios morosos. Una vez realizado esto, se obtienen todos los pagos pendientes del club, y por cada pago 
        /// pendiente, se verifica que la fecha actual sea mayor a la fecha de vencimiento del pago que se esté evaluando,
        /// y si la estructura que contendrá a los socios morosos, contiene al socio del que se está evaluando el pago. De
        /// cumplir con ambas condiciones, el socio es agregado como un socio moroso. Una vez verificados todos los pagos,
        /// se devuelve dicha estructura que contiene a los socios morosos.
        /// 
        /// Así, este método recibe una referencia a sí mismo (ReportesClubSemRust), y devuelve un tipo de dato Vec que 
        /// almacenaa los socios morosos (Vec(Socio)).
        #[ink(message)]
        pub fn listar_socios_morosos(&self) -> Vec<Socio> {
            if self.club.verificar_permisos_trait(){
                let pagos = self.club.get_pagos_totales_trait();
                let socios = self.club.get_socios_trait();
                return self.listar_socios_morosos_priv(pagos, socios);
            }
            panic!("No se cuenta con los permisos necesarios o la política de autorización se encuentra activada!");
        }

        fn listar_socios_morosos_priv(&self, pagos: Vec<Pago>, socios : Vec<Socio>) -> Vec<Socio>{
                let timestamp = self.env().block_timestamp();
                let pagos_pendientes = self.get_pagos_pendientes(&pagos);
                let mut vec_morosos : Vec<Socio> = Vec::new();
                let mut socio_pago; 
                for p in pagos_pendientes{
                    socio_pago = socios.iter().find(|s| s.get_dni() == p.get_dni_socio()).unwrap();
                    if p.get_fecha_venci() < timestamp && !vec_morosos.contains(socio_pago){
                        vec_morosos.push(socio_pago.clone());    
                    }
                }
                return vec_morosos;
        }

        /// Este método, se encarga de filtrar los pagos pendientes sobre los pagos totales del club, es decir, aquellos
        /// pagos que no tienen una fecha de pago, y devolverlos.
        /// 
        /// De esta forma, se crea una estructura que almacenará los pagos pendientes, y se obtienen en otra estructura 
        /// todos los pagos del club, desde la referencia que contiene este contrato. Así, se verifican todos los pagos del
        /// club, verificando que cada pago no contenga una fecha de pago, y si se cumple esta condición, el pago es
        /// agregado a la estructura que almacenará los pagos pendientes. Una vez verificados todos los pagos del club,
        /// se devuelve la estructura que contiene a todos los pagos pendientes.
        /// 
        /// Así, este método recibe una referencia a sí mismo (ReportesClubSemRust), y devuelve un tipo de dato Vec que 
        /// almacena los pagos pendientes (Vec(Pago)).
        fn get_pagos_pendientes(&self, pagos_vec : &Vec<Pago>) -> Vec<Pago> {
            let mut aux_vec : Vec<Pago> = Vec::new();
            for p in pagos_vec{
                if p.get_fecha_pago().is_none(){
                    aux_vec.push(p.clone());
                }
            }
            return aux_vec;
        }

        /// Este método, se encarga de filtrar los pagos realizados sobre los pagos totales del club, es decir, aquellos
        /// pagos que tienen una fecha de pago, y devolverlos.
        /// 
        /// De esta forma, se crea una estructura que almacenará los pagos realizados, y se obtienen en otra estructura 
        /// todos los pagos del club, desde la referencia que contiene este contrato. Así, se verifican todos los pagos del
        /// club, verificando que cada pago contenga una fecha de pago, y si se cumple esta condición, el pago es
        /// agregado a la estructura que almacenará los pagos realizados. Una vez verificados todos los pagos del club,
        /// se devuelve la estructura que contiene a todos los pagos realizados.
        /// 
        /// Así, este método recibe una referencia a sí mismo (ReportesClubSemRust), y devuelve un tipo de dato Vec que 
        /// almacena los pagos realizados (Vec(Pago))
        fn get_pagos_realizados(&self, pagos_vec : &Vec<Pago>) -> Vec<Pago> {
            let mut aux_vec : Vec<Pago> = Vec::new();
            for p in pagos_vec{
                if p.get_fecha_pago().is_some(){
                    aux_vec.push(p.clone());
                }
            }
            return aux_vec;
        }

        /// Este método, se encarga de realizar un informe de recaudación separado por categoría, indicando para cada 
        /// categoría, el monto recaudado. Para este caso, se tienen en cuanta aquellos pagos realizados.
        /// 
        /// De esta forma, se verifica que este contrato, contenga los permisos en el Club para realizar el reporte. De
        /// no contar con estos, arroja un panic informando lo sucedido. De lo contrario, se crean 2 estructuras: una
        /// que contendra los reportes por categoria, y otra en la que se tienen todos los pagos realizados de los socios.
        /// Así, se verifican todos los pagos realizados por los socios, tomando cada socio a partir del DNI presente en
        /// el pago, y de esta forma se verifica a qué categoría está sucripto al Club. Así, dependiendo a que categoría 
        /// pertenezca, se accede a la estructura que contiene la recaudación por categoría mediante su índice (el cual es
        /// reconocido dentro del método) y se incrementa el monto correspondiente a ese pago, en la categoría 
        /// correspondiente. Una vez verificados todos los pagos realizados, se devuelve la estructura que almacena la 
        /// recaudación total por categoría.
        /// 
        /// Así, este método recibe una referencia a sí mismo (ReportesClubSemRust), y devuelve un tipo de dato Vec que 
        /// almacena tuplas con dos datos: la categoría de socio, y el monto recaudado correspondiente a dicha categoria 
        /// (Vec((CategoriasSocios, u128))).
        #[ink(message)]
        pub fn generar_informe_recaudacion(&self) -> Vec<(CategoriasSocios, u128)> {
            if self.club.verificar_permisos_trait(){
                let pagos = self.club.get_pagos_totales_trait();
                let socios = self.club.get_socios_trait();
                return self.generar_informe_recaudacion_priv(pagos, socios);
            }
            return panic!("No se cuenta con los permisos necesarios o la política de autorización se encuentra activada!");
        }

        fn generar_informe_recaudacion_priv(&self, pagos : Vec<Pago>, socios : Vec<Socio>) -> Vec<(CategoriasSocios, u128)>{
                let tupla_a: (CategoriasSocios, u128) = (CategoriasSocios::A, 0);
                let tupla_b: (CategoriasSocios, u128) = (CategoriasSocios::B, 0);
                let tupla_c: (CategoriasSocios, u128) = (CategoriasSocios::C, 0);
                let mut vec_recaudacion = Vec::new();
                vec_recaudacion.push(tupla_a);
                vec_recaudacion.push(tupla_b);
                vec_recaudacion.push(tupla_c);
                let pagos_efectuados = self.get_pagos_realizados(&pagos);
                for pago in pagos_efectuados {
                    let dni = pago.get_dni_socio();
                    match socios.iter().find(|s| s.get_dni() == dni).unwrap().get_categoria() {
                        CategoriasSocios::A => vec_recaudacion[0].1 += pago.get_monto(),
                        CategoriasSocios::B => vec_recaudacion[1].1 += pago.get_monto(),
                        CategoriasSocios::C => vec_recaudacion[2].1 += pago.get_monto(),
                    }
                }
                return vec_recaudacion;
        }

        /// Este método se encarga de realizar un listado de socios no morosos, correspondiente a una actividad en especial, 
        /// devolviendo un listado de los socios no morosos en una actividad dada.
        /// 
        /// De esta forma, se verifica que este contrato, contenga los permisos en el Club para realizar el reporte. De
        /// no contar con estos, arroja un panic informando lo sucedido. De lo contrario, se verifica el ID de la actividad,
        /// verificando si el ID ingresado corresponde a algún deporte o al gimnasio. De corresponder a algún deporte, 
        /// se obtiene el deporte correspondiente al ID ingresado, a los socios no morosos (los cuales son aquellos que no
        /// tienen pagos vencidos sin pagar), los cuales se almacenan en una estructura, y se crea una estructura que 
        /// contendrá a los socios no morosos de la actividad dada. Así, se obtienen todos los socios desde el Club, y por
        /// cada socio, se verifica que la actividad escogida por el socio, sea la misma que la ingresada o que el socio
        /// este suscripto con la categoría A, ya que de ser así, tiene acceso a cualquier deporte, y que el socio no sea
        /// un socio moroso. De cumplir con ambas condiciones, se lo agrega a la estructura que almacena a los socios no 
        /// morosos. Una vez verificados todos los socios, se devuelve la estructura que almacena a los socios no morosos
        /// de la actividad ingresada. En caso de que el ID ingresado corresponda a la actividad gimnasio, se realizan los
        /// mismos pasos, sin obtener al deporte y sin tener en cuenta el deporte escogido por el socio o si es categoría
        /// A, ya que al gimnasio acceden todos los socios. Por lo tanto, se obtienen todos los socios, y los socios 
        /// morosos, y el socio es agregado al listado si no es moroso, y una vez verificados todos los socios, se devuelve 
        /// la estructura que contiene a los socios no morosos.
        /// 
        /// Por último, si el ID ingresado no corresponde a ninguna actividad, se arojará un panic informando la situación.
        /// 
        /// Así, este método recibe una referencia a sí mismo (ReportesClubSemRust) y un tipo de dato u32 que corresponde
        /// al ID correspondiente a la actividad sobre la cual se desea obtener los socios no morosos (id_deporte), y 
        /// devuelve un tipo de dato Vec que almacena los socios no morosos de la actividad ingresada (Vec(Socio)). 
        #[ink(message)]
        pub fn socios_no_morosos(&self, id_deporte: u32) -> Vec<Socio> {
            if self.club.verificar_permisos_trait(){
                let socios = self.club.get_socios_trait();
                return self.socios_no_morosos_priv(id_deporte, socios);
            }
            return panic!("No se cuenta con los permisos necesarios o la politica de autorizacion se encuentra activada!");
        }

        fn socios_no_morosos_priv(&self, id_deporte: u32, socios : Vec<Socio>) -> Vec<Socio>{
                if (id_deporte >= 1) && (id_deporte <= 7) {
                    let actividad = Deporte::deporte_from_id(&Some(id_deporte));
                    let vec_morosos = self.listar_socios_morosos();
                    let mut vec_no_morosos_actividad = Vec::new();
                    for s in &socios{
                        if ((s.get_deporte() == actividad) || (s.get_categoria() == CategoriasSocios::A)) && (!vec_morosos.contains(&s)) {
                            vec_no_morosos_actividad.push(s.clone());
                        }
                    } 
                    return vec_no_morosos_actividad;
                } else {
                    if id_deporte == 8 {
                        let vec_morosos = self.listar_socios_morosos();
                        let mut vec_no_morosos_actividad = Vec::new();
                        for s in &socios{
                            if !vec_morosos.contains(&s) {
                                vec_no_morosos_actividad.push(s.clone());
                            }
                        }
                        return vec_no_morosos_actividad; 
                    }
                    else {
                        return panic! ("El ID ingresado no corresponde a ninguna actividad!");
                    }
                }
        }
    }

    mod test {
        use super::*;

        #[ink::test]
        fn generar_informe_recaudacion_cat_a_test() {
            //cargo algunos socios categoria a 
            let bo = Socio::new("Duke Bouregard".to_string(), 320, CategoriasSocios::A, None);
            let luke = Socio::new("Duke Luke".to_string(), 321, CategoriasSocios::A, None);
            let daisy = Socio::new("Duke Daisy".to_string(), 63, CategoriasSocios::A, None);

            //creo los pagos: 1 pendiente y 2 pagos
            let pago_bo = Pago::new(320, 5000, 30, Some(10), false);
            let pago_luke = Pago::new(321, 5000, 30, Some(10), false);
            let pago_pend_daisy = Pago::new(63, 5000, 30, None, false);

            //creo los vec
            let mut pagos = Vec::new();
            pagos.push(pago_bo);
            pagos.push(pago_luke);
            pagos.push(pago_pend_daisy);

            let mut socios = Vec::new();
            socios.push(bo);
            socios.push(luke);
            socios.push(daisy);

            //creo los permisos
            let permisos = true;

            //creo a reportes y obtengo la recaudacion
            let struct_test = ClubTesting {pagos, socios, permisos};
            
            //llamo a la recaudacion
            let reportes = ReportesClubSemRust::new(struct_test);

            let recaudacion = reportes.generar_informe_recaudacion();
            //testeo segun lo esperado
            assert_eq!(recaudacion[0].1, 10000); 
            assert_eq!(recaudacion[1].1, 0); 
            assert_eq!(recaudacion[2].1, 0); 
        }

        #[ink::test]
        fn generar_informe_recaudacion_cat_b_test() {
            //cargo algunos socios categoria b 
            let bo = Socio::new("Duke Bouregard".to_string(), 320, CategoriasSocios::B, Some(Deporte::Futbol));
            let luke = Socio::new("Duke Luke".to_string(), 321, CategoriasSocios::B, Some(Deporte::Futbol));
            let daisy = Socio::new("Duke Daisy".to_string(), 63, CategoriasSocios::B, Some(Deporte::Natacion));

            //creo los pagos: 1 pendiente y 2 pagos
            let pago_bo = Pago::new(320, 3000, 30, Some(10), false);
            let pago_pend_luke = Pago::new(321, 3000, 30, None, false);
            let pago_daisy = Pago::new(63, 3000, 30, Some(10), false);

            //creo los vec
            let mut pagos = Vec::new();
            pagos.push(pago_bo);
            pagos.push(pago_pend_luke);
            pagos.push(pago_daisy);

            let mut socios = Vec::new();
            socios.push(bo);
            socios.push(luke);
            socios.push(daisy);

            //creo los permisos
            let permisos = true;

            //creo a reportes y obtengo la recaudacion
            let struct_test = ClubTesting {pagos, socios, permisos};
            
            //llamo a la recaudacion
            let reportes = ReportesClubSemRust::new(struct_test);

            let recaudacion = reportes.generar_informe_recaudacion();
            //testeo segun lo esperado
            assert_eq!(recaudacion[0].1, 0); 
            assert_eq!(recaudacion[1].1, 6000); 
            assert_eq!(recaudacion[2].1, 0); 
        }

        
        #[ink::test]
        fn generar_informe_recaudacion_cat_c_test() {
            //cargo algunos socios categoria c
            let bo = Socio::new("Duke Bouregard".to_string(), 320, CategoriasSocios::C, None);
            let luke = Socio::new("Duke Luke".to_string(), 321, CategoriasSocios::C, None);
            let daisy = Socio::new("Duke Daisy".to_string(), 63, CategoriasSocios::C, None);

            //creo los pagos: 1 pendiente y 2 pagos
            let pago_pend_bo = Pago::new(320, 2000, 30, None, false);
            let pago_luke = Pago::new(321, 2000, 30, Some(10), false);
            let pago_daisy = Pago::new(63, 2000, 30, Some(10), false);

            //creo los vec
            let mut pagos = Vec::new();
            pagos.push(pago_pend_bo);
            pagos.push(pago_luke);
            pagos.push(pago_daisy);

            let mut socios = Vec::new();
            socios.push(bo);
            socios.push(luke);
            socios.push(daisy);

            //creo los permisos
            let permisos = true;

            //creo a reportes y obtengo la recaudacion
            let struct_test = ClubTesting {pagos, socios, permisos};
            
            //llamo a la recaudacion
            let reportes = ReportesClubSemRust::new(struct_test);

            let recaudacion = reportes.generar_informe_recaudacion();
            
            //testeo segun lo esperado
            assert_eq!(recaudacion[0].1, 0); 
            assert_eq!(recaudacion[1].1, 0); 
            assert_eq!(recaudacion[2].1, 4000); 
        }

        #[ink::test]
        #[should_panic]
        fn socios_no_morosos_id_deporte_inexistente_test() {
            //cargo algunos socios categoria c
            let bo = Socio::new("Duke Bouregard".to_string(), 320, CategoriasSocios::C, None);
            let luke = Socio::new("Duke Luke".to_string(), 321, CategoriasSocios::C, None);
            let daisy = Socio::new("Duke Daisy".to_string(), 63, CategoriasSocios::C, None);

            //creo los socios
            let mut socios = Vec::new();
            socios.push(bo);
            socios.push(luke);
            socios.push(daisy);

            //creo los pagos
            let pagos = Vec::new();

            //creo los permisos
            let permisos = true;

            //creo a reportes y obtengo la recaudacion
            let struct_test = ClubTesting {pagos, socios, permisos};
            
            //llamo a la recaudacion
            let reportes = ReportesClubSemRust::new(struct_test);

            //llamo a socios_no_morosos con un id de deporte inexistente
            reportes.socios_no_morosos(10);
        }

        #[ink::test]
        fn socios_no_morosos_priv_id_deporte_entre_1_y_7_test() {
            //cargo algunos socios categoria c
            /*club.registrar_socio(320, "Bouregard Duke".to_string(), 3, None);
            club.registrar_socio(321, "Luke Duke".to_string(), 1, None);
            club.registrar_socio(63, "Daisy Duke".to_string(), 2, Some(6));*/

            //cargo algunos socios
            let bo = Socio::new("Duke Bouregard".to_string(), 320, CategoriasSocios::C, None);
            let luke = Socio::new("Duke Luke".to_string(), 321, CategoriasSocios::A, None);
            let daisy = Socio::new("Duke Daisy".to_string(), 63, CategoriasSocios::B, Some(Deporte::Tenis));

            //creo los socios
            let mut socios = Vec::new();
            socios.push(bo);
            socios.push(luke);
            socios.push(daisy);

            //creo los pagos
            let pagos = Vec::new();

            //creo los permisos
            let permisos = true;

            //creo a reportes y obtengo la recaudacion
            let struct_test = ClubTesting {pagos, socios, permisos};

            //creo a reportes
            let reportes = ReportesClubSemRust::new(struct_test);

            let no_morosos_id_1 = reportes.socios_no_morosos(1);
            let no_morosos_id_2 = reportes.socios_no_morosos(2);
            let no_morosos_id_3 = reportes.socios_no_morosos(3);
            let no_morosos_id_4 = reportes.socios_no_morosos(4);
            let no_morosos_id_5 = reportes.socios_no_morosos(5);
            let no_morosos_id_6 = reportes.socios_no_morosos(6);
            let no_morosos_id_7 = reportes.socios_no_morosos(7);

            //testeo cada caso
            assert_eq! (no_morosos_id_1.len(), 1);
            assert_eq! (no_morosos_id_2.len(), 1);
            assert_eq! (no_morosos_id_3.len(), 1);
            assert_eq! (no_morosos_id_4.len(), 1);
            assert_eq! (no_morosos_id_5.len(), 1);
            assert_eq! (no_morosos_id_6.len(), 2);
            assert_eq! (no_morosos_id_7.len(), 1);
        }

        #[ink::test]
        fn socios_no_morosos_priv_id_deporte_8_test() {
            //cargo algunos socios
            let bo = Socio::new("Duke Bouregard".to_string(), 320, CategoriasSocios::C, None);
            let luke = Socio::new("Duke Luke".to_string(), 321, CategoriasSocios::A, None);
            let daisy = Socio::new("Duke Daisy".to_string(), 63, CategoriasSocios::B, Some(Deporte::Natacion));

            //creo los socios
            let mut socios = Vec::new();
            socios.push(bo);
            socios.push(luke);
            socios.push(daisy);

            //creo los pagos
            let pagos = Vec::new();

            //creo los permisos
            let permisos = true;

            //creo a reportes y obtengo la recaudacion
            let struct_test = ClubTesting {pagos, socios, permisos};

            //creo a reportes
            let reportes = ReportesClubSemRust::new(struct_test);

            //obtengo los no morosos con id 8
            let no_morosos_id_8 = reportes.socios_no_morosos(8);

            //testeo
            assert_eq! (no_morosos_id_8.len(), 3);
        }

        ///para este test, se crean 3 socios y se realizan 7 pagos diferentes, de los cuales, los unicos que estan realizados son el
        /// p1,p3,p5,p6 y p7, creo un vec con los pagos a enviar y un vec con los pagos realizados, posteriormente invoco a la funcion pagos realizados
        /// para que devuelva el resultado en un vec llamado vec_devuelto. Finalmente comparo el vec_devuelto con el vec que cree solo con los pagos realizados
        #[ink::test]
        fn get_pagos_realizados_test(){
            let bo = Socio::new("Duke Bouregard".to_string(), 320, CategoriasSocios::A, None);
            let luke = Socio::new("Duke Luke".to_string(), 321, CategoriasSocios::A, None);
            let daisy = Socio::new("Duke Daisy".to_string(), 63, CategoriasSocios::A, None);
            let mut socios = Vec::new();
            socios.push(bo);
            socios.push(luke);
            socios.push(daisy);
            let p1 = Pago::new(320,5000,6000,Some(5000),false);
            let p2 = Pago::new(63,2000,6000,None,false);
            let p3 = Pago::new(63,2000,6000,Some(5000),false);
            let p4 = Pago::new(320,1000,6000,None,false);
            let p5 = Pago::new(320,1000,6000,Some(5000),false);
            let p6 = Pago::new(321,5000,6000,Some(5000),false);
            let p7 = Pago::new(321,5000,6000,Some(4700),true);
            let mut vec_enviar:Vec<Pago> = Vec::new();
            vec_enviar.push(p1.clone());
            vec_enviar.push(p2.clone());
            vec_enviar.push(p3.clone());
            vec_enviar.push(p4.clone());
            vec_enviar.push(p5.clone());
            vec_enviar.push(p6.clone());
            vec_enviar.push(p7.clone());
            let mut vec_esperado:Vec<Pago> = Vec::new();
            vec_esperado.push(p1);
            vec_esperado.push(p3);
            vec_esperado.push(p5);
            vec_esperado.push(p6);
            vec_esperado.push(p7);
            let permisos = true;
            let pagos = vec_enviar.clone();
            let struct_test = ClubTesting {pagos, socios, permisos};
            let reportes = ReportesClubSemRust::new(struct_test);
            let vec_devuelto = reportes.get_pagos_realizados(&vec_enviar);
            assert_eq!(vec_devuelto,vec_esperado);
        }

        ///inicializo el timestamp actual en 8500
        ///primero creo 3 socios y los agrego a un vec llamado socios, luego 7 pagos de los cuales 3 son pendientes
        /// y uno solo de ellos esta pendiente con fecha de vencimiento posterior al timestampacutal(pago6)
        /// luego creo el vec de pagos, y un vec en el cual agrego los socios que son morosos (vec_esperado)
        /// por ultimo invoco a la funcion listar_socios_morosos y lo que devuelve me lo guardo en vec_devuelto
        /// finalmente, comparo el vec_devuelto con el vec_esperado y compruebo que estos son iguales
        #[ink::test]
        fn listar_socios_morosos_priv_test(){
            let tiempo_ficticio:u64 = 8500; 
            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(tiempo_ficticio);
            let bo = Socio::new("Duke Bouregard".to_string(), 320, CategoriasSocios::A, None);
            let luke = Socio::new("Duke Luke".to_string(), 321, CategoriasSocios::A, None);
            let daisy = Socio::new("Duke Daisy".to_string(), 63, CategoriasSocios::A, None);
            let mut socios = Vec::new();
            socios.push(bo.clone());
            socios.push(luke.clone());
            socios.push(daisy.clone());
            let p1 = Pago::new(320,5000,6000,Some(5000),false);
            let p2 = Pago::new(63,2000,6000,None,false);
            let p3 = Pago::new(63,2000,6000,Some(5000),false);
            let p4 = Pago::new(320,1000,3000,None,false);
            let p5 = Pago::new(320,1000,3000,Some(2000),false);
            let p6 = Pago::new(321,5000,9000,None,false);
            let p7 = Pago::new(321,5000,6000,Some(4700),true);
            let mut vec_enviar:Vec<Pago> = Vec::new();
            vec_enviar.push(p1.clone());
            vec_enviar.push(p2.clone());
            vec_enviar.push(p3.clone());
            vec_enviar.push(p4.clone());
            vec_enviar.push(p5.clone());
            vec_enviar.push(p6.clone());
            vec_enviar.push(p7.clone());
            let mut vec_esperado:Vec<Socio> = Vec::new();
            vec_esperado.push(daisy.clone());
            vec_esperado.push(bo.clone());
            let permisos = true;
            let pagos = vec_enviar.clone();
            let struct_test = ClubTesting {pagos, socios, permisos};
            let reportes = ReportesClubSemRust::new(struct_test);
            let vec_devuelto = reportes.listar_socios_morosos();
            assert_eq!(vec_devuelto,vec_esperado);
        }

        #[ink::test]
        fn get_pagos_realizados_vacio_test(){
            // Inicializo los valores que tomaran los campos de ClubTesting
            let permisos = true; 
            let pagos : Vec<Pago> = Vec::new();
            let socios : Vec<Socio> = Vec::new();
            // Inicializo ClubTesting con los valores previamente instanciados
            let club = ClubTesting{permisos, pagos, socios};
            // Inicializo ReportesClubSemRust
            let reportes = ReportesClubSemRust::new(club);
            // Inicializo el vector de los elementos esperados
            let vec_esperado = Vec::new();
            // Compruebo que el resultado sea el esperado
            assert_eq!(reportes.get_pagos_realizados(&reportes.club.pagos), vec_esperado);
        }

        #[ink::test]
        fn get_pagos_realizados_cargado_test(){
            let permisos = true;
            // Inicializo pagos
            let p1 = Pago::new(111, 10000, 20000, None, false);
            let p2 = Pago::new(333, 3000, 15000, Some(16500), false);
            let p3 = Pago::new(555, 10000, 20000, Some(10000), false);
            let p4 = Pago::new(222, 2500, 20000, None, true);
            let p5 = Pago::new(989, 10000, 20000, None, false);
            let p6 = Pago::new(111, 10000, 50000, Some(75000), false);
            let p7 = Pago::new(643, 2500, 50000, None, true);
            // Creo los vectores de pagos necesarios (El que adopta ClubTesting y el que almacena el resultado esperado)
            let mut pagos : Vec<Pago> = Vec::new();
            let mut vec_esperado = Vec::new();
            // Pusheo los valores esperados al vector del resultado coherente
            vec_esperado.push(p2.clone());
            vec_esperado.push(p3.clone());
            vec_esperado.push(p6.clone());
            // Pusheo todos los pagos al vector de pagos que adoptará ClubTesting
            pagos.push(p1);
            pagos.push(p2);
            pagos.push(p3);
            pagos.push(p4);
            pagos.push(p5);
            pagos.push(p6);
            pagos.push(p7);
            // Inicializo vector de socios, ClubTesting y el ReportesClubSemRust
            let socios : Vec<Socio> = Vec::new();
            let club = ClubTesting{permisos, pagos, socios};
            let reportes = ReportesClubSemRust::new(club);
            // Compruebo que el resultado sea el esperado
            assert_eq!(reportes.get_pagos_realizados(&reportes.club.pagos), vec_esperado);
        }

        #[ink::test]
        fn get_pagos_realizados_cargado_todos_realizados_test(){
            let permisos = true;
            // Inicializo pagos
            let p1 = Pago::new(111, 10000, 20000, Some(13000), false);
            let p2 = Pago::new(333, 3000, 15000, Some(16500), false);
            let p3 = Pago::new(555, 10000, 20000, Some(10000), false);
            let p4 = Pago::new(222, 2500, 20000, Some(20000), true);
            let p5 = Pago::new(989, 10000, 20000, Some(6000), false);
            let p6 = Pago::new(111, 10000, 50000, Some(75000), false);
            let p7 = Pago::new(643, 2500, 50000, Some(50000), true);
            // Creo los vectores de pagos necesarios (El que adopta ClubTesting y el que almacena el resultado esperado)
            let mut pagos : Vec<Pago> = Vec::new();
            // Inicializo el vector esperado clonando el pagos previamente instanciado
            let mut vec_esperado = Vec::new();
            // Pusheo todos los pagos al vector de pagos que adoptará ClubTesting
            vec_esperado.push(p1.clone());
            pagos.push(p1);
            vec_esperado.push(p2.clone());
            pagos.push(p2);
            vec_esperado.push(p3.clone());
            pagos.push(p3);
            vec_esperado.push(p4.clone());
            pagos.push(p4);
            vec_esperado.push(p5.clone());
            pagos.push(p5);
            vec_esperado.push(p6.clone());
            pagos.push(p6);
            vec_esperado.push(p7.clone());
            pagos.push(p7);
            // Inicializo vector de socios, ClubTesting y el ReportesClubSemRust
            let socios : Vec<Socio> = Vec::new();
            let club = ClubTesting{permisos, pagos, socios};
            let reportes = ReportesClubSemRust::new(club);
            // Compruebo que el resultado sea el esperado
            assert_eq!(reportes.get_pagos_realizados(&reportes.club.pagos),vec_esperado);
        }

        #[ink::test]
        fn get_pagos_realizados_cargado_sin_realizados_test(){
            let permisos = true;
            // Inicializo pagos
            let p1 = Pago::new(111, 10000, 20000, None, false);
            let p2 = Pago::new(333, 3000, 15000, None, false);
            let p3 = Pago::new(555, 10000, 20000, None, false);
            let p4 = Pago::new(222, 2500, 20000, None, true);
            let p5 = Pago::new(989, 10000, 20000, None, false);
            let p6 = Pago::new(111, 10000, 50000, None, false);
            let p7 = Pago::new(643, 2500, 50000, None, true);
            // Creo el vector de pagos que adoptará el ClubTesting
            let mut pagos : Vec<Pago> = Vec::new();
            // Instancio el vector vacio esperado para el caso unitario
            let vec_esperado = Vec::new();
            // Pusheo todos los pagos al vector de pagos que adoptará ClubTesting
            pagos.push(p1);
            pagos.push(p2);
            pagos.push(p3);
            pagos.push(p4);
            pagos.push(p5);
            pagos.push(p6);
            pagos.push(p7);
            // Inicializo vector de socios, ClubTesting y el ReportesClubSemRust
            let socios : Vec<Socio> = Vec::new();
            let club = ClubTesting{permisos, pagos, socios};
            let reportes = ReportesClubSemRust::new(club);
            // Compruebo que el resultado sea el esperado
            assert_eq!(reportes.get_pagos_realizados(&reportes.club.pagos), vec_esperado);
        }

        ///para este test, se crean 3 socios y se realizan 7 pagos diferentes, de los cuales, los unicos que estan realizados son el
        /// p1,p3,p5,p6 y p7, creo un vec con los pagos a enviar y un vec con los pagos realizados, posteriormente invoco a la funcion pagos realizados
        /// para que devuelva el resultado en un vec llamado vec_devuelto. Finalmente comparo el vec_devuelto con el vec que cree solo con los pagos realizados
        #[ink::test]
        fn get_pagos_realizados_generico_test(){
            let bo = Socio::new("Duke Bouregard".to_string(), 320, CategoriasSocios::A, None);
            let luke = Socio::new("Duke Luke".to_string(), 321, CategoriasSocios::A, None);
            let daisy = Socio::new("Duke Daisy".to_string(), 63, CategoriasSocios::A, None);
            let mut socios = Vec::new();
            socios.push(bo);
            socios.push(luke);
            socios.push(daisy);
            let p1 = Pago::new(320,5000,6000,Some(5000),false);
            let p2 = Pago::new(63,2000,6000,None,false);
            let p3 = Pago::new(63,2000,6000,Some(5000),false);
            let p4 = Pago::new(320,1000,6000,None,false);
            let p5 = Pago::new(320,1000,6000,Some(5000),false);
            let p6 = Pago::new(321,5000,6000,Some(5000),false);
            let p7 = Pago::new(321,5000,6000,Some(4700),true);
            let mut vec_enviar:Vec<Pago> = Vec::new();
            vec_enviar.push(p1.clone());
            vec_enviar.push(p2.clone());
            vec_enviar.push(p3.clone());
            vec_enviar.push(p4.clone());
            vec_enviar.push(p5.clone());
            vec_enviar.push(p6.clone());
            vec_enviar.push(p7.clone());
            let mut vec_esperado:Vec<Pago> = Vec::new();
            vec_esperado.push(p1);
            vec_esperado.push(p3);
            vec_esperado.push(p5);
            vec_esperado.push(p6);
            vec_esperado.push(p7);
            let permisos = true;
            let pagos = vec_enviar.clone();
            let struct_test = ClubTesting {pagos, socios, permisos};
            let reportes = ReportesClubSemRust::new(struct_test);
            let vec_devuelto = reportes.get_pagos_realizados(&vec_enviar);
            assert_eq!(vec_devuelto,vec_esperado);
        }

        #[ink::test]
        fn get_pagos_pendientes_vacio_test() {
            // Inicializo los valores que tomaran los campos de ClubTesting
            let permisos = true; 
            let pagos : Vec<Pago> = Vec::new();
            let socios : Vec<Socio> = Vec::new();
            // Inicializo ClubTesting con los valores previamente instanciados
            let club = ClubTesting{permisos, pagos, socios};
            // Inicializo ReportesClubSemRust
            let reportes = ReportesClubSemRust::new(club);
            // Inicializo el vector de los elementos esperados
            let vec_esperado = Vec::new();
            // Compruebo que el resultado sea el esperado
            assert_eq!(reportes.get_pagos_pendientes(&reportes.club.pagos), vec_esperado);
        }

        #[ink::test]
        fn get_pagos_pendientes_cargado_test() {
            let permisos = true;
            // Inicializo pagos
            let p1 = Pago::new(111, 10000, 20000, None, false);
            let p2 = Pago::new(333, 3000, 15000, Some(16500), false);
            let p3 = Pago::new(555, 10000, 20000, Some(10000), false);
            let p4 = Pago::new(222, 2500, 20000, None, true);
            let p5 = Pago::new(989, 10000, 20000, None, false);
            let p6 = Pago::new(111, 10000, 50000, Some(75000), false);
            let p7 = Pago::new(643, 2500, 50000, None, true);
            // Creo los vectores de pagos necesarios (El que adopta ClubTesting y el que almacena el resultado esperado)
            let mut pagos : Vec<Pago> = Vec::new();
            let mut vec_esperado = Vec::new();
            // Pusheo los valores esperados al vector del resultado coherente
            vec_esperado.push(p1.clone());
            vec_esperado.push(p4.clone());
            vec_esperado.push(p5.clone());
            vec_esperado.push(p7.clone());
            // Pusheo todos los pagos al vector de pagos que adoptará ClubTesting
            pagos.push(p1);
            pagos.push(p2);
            pagos.push(p3);
            pagos.push(p4);
            pagos.push(p5);
            pagos.push(p6);
            pagos.push(p7);
            // Inicializo vector de socios, ClubTesting y el ReportesClubSemRust
            let socios : Vec<Socio> = Vec::new();
            let club = ClubTesting{permisos, pagos, socios};
            let reportes = ReportesClubSemRust::new(club);
            // Compruebo que el resultado sea el esperado
            assert_eq!(reportes.get_pagos_pendientes(&reportes.club.pagos), vec_esperado);
        }

        #[ink::test]
        fn get_pagos_pendientes_todos_realizados_test () {
            let permisos = true;
            // Inicializo pagos
            let p1 = Pago::new(111, 10000, 20000, Some(13000), false);
            let p2 = Pago::new(333, 3000, 15000, Some(16500), false);
            let p3 = Pago::new(555, 10000, 20000, Some(10000), false);
            let p4 = Pago::new(222, 2500, 20000, Some(20000), true);
            let p5 = Pago::new(989, 10000, 20000, Some(6000), false);
            let p6 = Pago::new(111, 10000, 50000, Some(75000), false);
            let p7 = Pago::new(643, 2500, 50000, Some(50000), true);
            // Creo los vectores de pagos necesarios (El que adopta ClubTesting y el que almacena el resultado esperado)
            let mut pagos : Vec<Pago> = Vec::new();
            // Inicializo el vector esperado
            let vec_esperado = Vec::new();
            // Pusheo todos los pagos al vector de pagos que adoptará ClubTesting
            pagos.push(p1);
            pagos.push(p2);
            pagos.push(p3);
            pagos.push(p4);
            pagos.push(p5);
            pagos.push(p6);
            pagos.push(p7);
            // Inicializo vector de socios, ClubTesting y el ReportesClubSemRust
            let socios : Vec<Socio> = Vec::new();
            let club = ClubTesting{permisos, pagos, socios};
            let reportes = ReportesClubSemRust::new(club);
            // Compruebo que el resultado sea el esperado
            assert_eq!(reportes.get_pagos_pendientes(&reportes.club.pagos),vec_esperado);
        }

        #[ink::test]
        fn get_pagos_pendientes_todos_pendientes_test() {
            let permisos = true;
            // Inicializo pagos
            let p1 = Pago::new(111, 10000, 20000, None, false);
            let p2 = Pago::new(333, 3000, 15000, None, false);
            let p3 = Pago::new(555, 10000, 20000, None, false);
            let p4 = Pago::new(222, 2500, 20000, None, true);
            let p5 = Pago::new(989, 10000, 20000, None, false);
            let p6 = Pago::new(111, 10000, 50000, None, false);
            let p7 = Pago::new(643, 2500, 50000, None, true);
            // Creo el vector de pagos que adoptará el ClubTesting
            let mut pagos : Vec<Pago> = Vec::new();
            // Inicializo el vector esperado clonando el pagos previamente instanciado
            let mut vec_esperado = Vec::new();
            // Pusheo todos los pagos al vector de pagos que adoptará ClubTesting
            vec_esperado.push(p1.clone());
            pagos.push(p1);
            vec_esperado.push(p2.clone());
            pagos.push(p2);
            vec_esperado.push(p3.clone());
            pagos.push(p3);
            vec_esperado.push(p4.clone());
            pagos.push(p4);
            vec_esperado.push(p5.clone());
            pagos.push(p5);
            vec_esperado.push(p6.clone());
            pagos.push(p6);
            vec_esperado.push(p7.clone());
            pagos.push(p7);
            // Inicializo vector de socios, ClubTesting y el ReportesClubSemRust
            let socios : Vec<Socio> = Vec::new();
            let club = ClubTesting{permisos, pagos, socios};
            let reportes = ReportesClubSemRust::new(club);
            // Compruebo que el resultado sea el esperado
            assert_eq!(reportes.get_pagos_pendientes(&reportes.club.pagos), vec_esperado);
        }

        ///para este test, se crean 3 socios y se realizan 7 pagos diferentes, de los cuales, los unicos que estan realizados son el
        /// p1,p3,p5,p6 y p7, creo un vec con los pagos a enviar y un vec con los pagos pendientes, posteriormente invoco a la funcion pagos pendientes
        /// para que devuelva el resultado en un vec llamado vec_devuelto. Finalmente comparo el vec_devuelto con el vec que cree solo con los pagos pendienetes
        #[ink::test]
        fn get_pagos_pendientes_generico_test(){
            let bo = Socio::new("Duke Bouregard".to_string(), 320, CategoriasSocios::A, None);
            let luke = Socio::new("Duke Luke".to_string(), 321, CategoriasSocios::A, None);
            let daisy = Socio::new("Duke Daisy".to_string(), 63, CategoriasSocios::A, None);
            let mut socios = Vec::new();
            socios.push(bo);
            socios.push(luke);
            socios.push(daisy);
            let p1 = Pago::new(320,5000,6000,Some(5000),false);
            let p2 = Pago::new(63,2000,6000,None,false);
            let p3 = Pago::new(63,2000,6000,Some(5000),false);
            let p4 = Pago::new(320,1000,6000,None,false);
            let p5 = Pago::new(320,1000,6000,Some(5000),false);
            let p6 = Pago::new(321,5000,6000,Some(5000),false);
            let p7 = Pago::new(321,5000,6000,Some(4700),true);
            let mut vec_enviar:Vec<Pago> = Vec::new();
            vec_enviar.push(p1.clone());
            vec_enviar.push(p2.clone());
            vec_enviar.push(p3.clone());
            vec_enviar.push(p4.clone());
            vec_enviar.push(p5.clone());
            vec_enviar.push(p6.clone());
            vec_enviar.push(p7.clone());
            let mut vec_esperado:Vec<Pago> = Vec::new();
            vec_esperado.push(p2);
            vec_esperado.push(p4);
            let permisos = true;
            let pagos = vec_enviar.clone();
            let struct_test = ClubTesting {pagos, socios, permisos};
            let reportes = ReportesClubSemRust::new(struct_test);
            let vec_devuelto = reportes.get_pagos_pendientes(&vec_enviar);
            assert_eq!(vec_devuelto,vec_esperado);
        }

    }
}