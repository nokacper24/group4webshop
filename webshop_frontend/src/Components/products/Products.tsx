import { useEffect, useState } from "react";
import { ProductCard, ProductCardProps } from "./ProductCard";

let baseUrl = "http://localhost:8081";
export type State = {
  products: ProductCardProps[];
};

export default function Products() {

    const [products, setProducts] = useState<ProductCardProps[]>([]);

    const fetchProducts = async () => {
        const response = await fetch(`${baseUrl}/api/products`);
        const data = await response.json();
        const products = data.map((product: any) => {
            return {
                props: {
                    product_id: product.product_id,
                    name: product.display_name,
                    description: product.short_description,
                    sourceImage: product.main_image,
                },
            };
        });

        setProducts(products);
    };

    useEffect(() => {
        fetchProducts();
    }, []);
    

  return (
    <div>
      <section className="container">
        <h1>Our solutions</h1>
        <ul className="product-list grid-container">
          {products.map((product) => (
            <ProductCard key={product.props.name} props={product.props} />
          ))}
        </ul>
      </section>
    </div>
  );
}
