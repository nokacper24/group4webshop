// @generated automatically by Diesel CLI.

diesel::table! {
    orders (order_id) {
        order_id -> Int4,
        user_id -> Int4,
        order_date -> Date,
        status_id -> Int4,
    }
}

diesel::table! {
    product (product_id) {
        product_id -> Int4,
        name -> Varchar,
        description -> Varchar,
        price -> Numeric,
        amount_available -> Int4,
        main_image -> Bytea,
    }
}

diesel::table! {
    productorder (product_order_id) {
        product_order_id -> Int4,
        product_id -> Int4,
        order_id -> Int4,
        amount -> Int4,
    }
}

diesel::table! {
    role (role_id) {
        role_id -> Int4,
        role -> Varchar,
    }
}

diesel::table! {
    status (status_id) {
        status_id -> Int4,
        status -> Varchar,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
        role_id -> Int4,
    }
}

diesel::joinable!(orders -> status (status_id));
diesel::joinable!(orders -> users (user_id));
diesel::joinable!(productorder -> orders (order_id));
diesel::joinable!(productorder -> product (product_id));
diesel::joinable!(users -> role (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    orders,
    product,
    productorder,
    role,
    status,
    users,
);
