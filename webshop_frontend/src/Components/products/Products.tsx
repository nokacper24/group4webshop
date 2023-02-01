import { Product } from "./Product";

export default function Products() {
    let state = {
        products: [
            {name: 'Acme', description: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.', sourceImage: 'https://unsplash.it/200/100'},
            {name: 'Acme2', description: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.', sourceImage: 'https://unsplash.it/200/100'},
            {name: 'Acme3', description: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.', sourceImage: 'https://unsplash.it/200/100'},
            {name: 'Acme4', description: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.', sourceImage: 'https://unsplash.it/200/100'},
        ]
    }
    return (
        <div>
            <section className="container">
                <h1>Our solutions</h1>
                <ul className="product-list grid-container">
                    {
                        state.products.map((product) => (
                            <Product 
                                key={product.name}
                                product={product} />
                        ))
                    }
                </ul>
            </section>
        </div>
    );
}