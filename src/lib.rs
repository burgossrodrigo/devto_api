pub mod domain {

    pub mod use_case {
        pub mod api_call;
        pub mod get_posts;
        pub mod use_case;
    }

    pub mod entities {
        pub mod post;
    }
}

pub mod infra {
    pub mod handler {
        pub mod post;
    }
}

pub mod use_case {
    pub mod api_call;
}