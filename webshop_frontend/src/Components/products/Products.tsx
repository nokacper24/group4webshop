import { Product, ProductProps} from "./Product";

export type State = {
    products: ProductProps[];
}

export default function Products() {
    const website_data = [
        {name: 'Acme', description: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.', sourceImage: 'https://unsplash.it/200/100'},
        {name: 'Acme2', description: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.', sourceImage: 'https://unsplash.it/200/100'},
        {name: 'Acme3', description: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.', sourceImage: 'https://unsplash.it/200/100'},
        {name: 'Acme4', description: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.', sourceImage: 'https://unsplash.it/200/100'},
    ]
    let state: State = {
        products: [],
    }
    website_data.forEach((projectObject) => {
        let newProduct: ProductProps = {
            props: projectObject, 
        }
        state.products.push(newProduct);
    });

    return (
        <div>
            <section className="container">
                <h1>Our solutions</h1>
                <ul className="product-list grid-container">
                    {
                        state.products.map((product) => (
                            <Product 
                                key={product.props.name}
                                props={product.props} />
                        ))
                    }
                </ul>
            </section>
        </div>
    );
}