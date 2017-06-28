use serde_json::error::Error as SerdeError;

error_chain!{
    links {
    }

    foreign_links {
        Io(::std::io::Error) #[cfg(unix)];
        Json(SerdeError);
    }
}
