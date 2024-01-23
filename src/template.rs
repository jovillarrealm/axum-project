/// Operaciones con la base de datos
/// Es un cluster de mongo atlas que requiere un uri que se trata de leer de un secret.txt
pub mod db;
/// Descarga y actualiza archivos del portal de meddata.
/// usa reqwuest para hacer esas querys
pub mod med_data;
/// Templates de los ejemplos 
pub mod user;
/// Operaciones de los ejemplos, usa el mod user
pub mod user_fn;
