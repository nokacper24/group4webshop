import { useEffect, useState } from "react";
import { Product } from "../../Interfaces";
import { ProductCard, ProductCardProps } from "./ProductCard";
import { fetchProducts } from "../../ApiController";

export default function Products() {
  const [products, setProducts] = useState<Product[]>([]);

  useEffect(() => {
    fetchProducts().then((products: Product[]) => setProducts(products));
  }, []);

  return (
    <>
      <section className="container">
        <h1>Our solutions</h1>
        <ul className="product-list grid-container">
          {products.map((product) => (
            <ProductCard key={product.display_name} product={product} />
          ))}
        </ul>
      </section>
    </>
  );
}
