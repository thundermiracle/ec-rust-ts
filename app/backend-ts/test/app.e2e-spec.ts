/* eslint-disable @typescript-eslint/no-unsafe-member-access */
/* eslint-disable @typescript-eslint/no-unsafe-argument */
import { Test, TestingModule } from '@nestjs/testing';
import { INestApplication, ValidationPipe } from '@nestjs/common';
import * as request from 'supertest';
import { AppModule } from './../src/app.module';

describe('E-commerce API (e2e)', () => {
  let app: INestApplication;

  beforeEach(async () => {
    const moduleFixture: TestingModule = await Test.createTestingModule({
      imports: [AppModule],
    }).compile();

    app = moduleFixture.createNestApplication();

    // Apply same configuration as main.ts
    app.useGlobalPipes(
      new ValidationPipe({
        whitelist: true,
        forbidNonWhitelisted: true,
        transform: true,
      }),
    );

    await app.init();
  });

  afterEach(async () => {
    await app.close();
  });

  describe('Health Check', () => {
    it('should return 404 for root path (no controller)', () => {
      return request(app.getHttpServer()).get('/').expect(404);
    });
  });

  describe('Categories API', () => {
    it('GET /categories should return category list', async () => {
      const response = await request(app.getHttpServer())
        .get('/categories')
        .expect(200);

      expect(response.body).toHaveProperty('categories');
      expect(Array.isArray(response.body.categories)).toBe(true);
    });
  });

  describe('Colors API', () => {
    it('GET /colors should return color list', async () => {
      const response = await request(app.getHttpServer())
        .get('/colors')
        .expect(200);

      expect(response.body).toHaveProperty('colors');
      expect(Array.isArray(response.body.colors)).toBe(true);
    });
  });

  describe('Shipping Methods API', () => {
    it('GET /shipping-methods should return shipping method list', async () => {
      const response = await request(app.getHttpServer())
        .get('/shipping-methods')
        .expect(200);

      expect(response.body).toHaveProperty('shippingMethods');
      expect(Array.isArray(response.body.shippingMethods)).toBe(true);
    });
  });

  describe('Payment Methods API', () => {
    it('GET /payment-methods should return payment method list', async () => {
      const response = await request(app.getHttpServer())
        .get('/payment-methods')
        .expect(200);

      expect(response.body).toHaveProperty('paymentMethods');
      expect(Array.isArray(response.body.paymentMethods)).toBe(true);
    });
  });

  describe('Products API', () => {
    it('GET /products should return product list', async () => {
      const response = await request(app.getHttpServer())
        .get('/products')
        .expect(200);

      expect(response.body).toHaveProperty('products');
      expect(response.body).toHaveProperty('total');
      expect(response.body).toHaveProperty('page');
      expect(response.body).toHaveProperty('limit');
      expect(Array.isArray(response.body.products)).toBe(true);
    });

    it('GET /products with pagination should work', async () => {
      const response = await request(app.getHttpServer())
        .get('/products?page=1&limit=5')
        .expect(200);

      expect(response.body.page).toBe(1);
      expect(response.body.limit).toBe(5);
    });

    it('GET /products/:id with invalid UUID should return 400', async () => {
      return request(app.getHttpServer())
        .get('/products/invalid-uuid')
        .expect(400);
    });
  });

  describe('Variants API', () => {
    it('POST /variants should accept SKU IDs array', async () => {
      const validUuid = '123e4567-e89b-12d3-a456-426614174000';

      const response = await request(app.getHttpServer())
        .post('/variants')
        .send({ skuIds: [validUuid] })
        .expect(200);

      expect(Array.isArray(response.body)).toBe(true);
    });

    it('POST /variants with empty array should return empty result', async () => {
      const response = await request(app.getHttpServer())
        .post('/variants')
        .send({ skuIds: [] })
        .expect(200);

      expect(response.body).toEqual([]);
    });

    it('POST /variants with invalid request should return 400', async () => {
      return request(app.getHttpServer())
        .post('/variants')
        .send({ invalidField: 'test' })
        .expect(400);
    });
  });

  describe('Cart API', () => {
    it('POST /cart should validate request structure', async () => {
      // Test with missing required fields
      return request(app.getHttpServer()).post('/cart').send({}).expect(400);
    });

    it('POST /cart with valid structure should process', async () => {
      const validCartRequest = {
        items: [
          {
            skuId: '123e4567-e89b-12d3-a456-426614174000',
            quantity: 1,
          },
        ],
        shippingMethodId: 'standard-shipping',
        paymentMethodId: 'credit-card',
      };

      // This might return 404 if SKU doesn't exist, which is expected behavior
      const response = await request(app.getHttpServer())
        .post('/cart')
        .send(validCartRequest);

      expect([200, 404, 422]).toContain(response.status);
    });

    it('POST /cart with invalid quantity should return 400', async () => {
      const invalidCartRequest = {
        items: [
          {
            skuId: '123e4567-e89b-12d3-a456-426614174000',
            quantity: 0, // Invalid quantity
          },
        ],
        shippingMethodId: 'standard-shipping',
        paymentMethodId: 'credit-card',
      };

      return request(app.getHttpServer())
        .post('/cart')
        .send(invalidCartRequest)
        .expect(400);
    });
  });

  describe('Orders API', () => {
    it('POST /orders should validate request structure', async () => {
      return request(app.getHttpServer()).post('/orders').send({}).expect(400);
    });

    it('POST /orders with invalid email should return 400', async () => {
      const invalidOrderRequest = {
        items: [
          {
            skuId: '123e4567-e89b-12d3-a456-426614174000',
            quantity: 1,
          },
        ],
        customerInfo: {
          firstName: '太郎',
          lastName: '田中',
          email: 'invalid-email', // Invalid email format
          phone: '090-1234-5678',
        },
        shippingAddress: {
          postalCode: '123-4567',
          prefecture: '東京都',
          city: '渋谷区',
          streetAddress: '渋谷1-1-1',
        },
        shippingMethodId: 'standard-shipping',
        paymentMethodId: 'credit-card',
      };

      return request(app.getHttpServer())
        .post('/orders')
        .send(invalidOrderRequest)
        .expect(400);
    });

    it('POST /orders with invalid postal code should return 400', async () => {
      const invalidOrderRequest = {
        items: [
          {
            skuId: '123e4567-e89b-12d3-a456-426614174000',
            quantity: 1,
          },
        ],
        customerInfo: {
          firstName: '太郎',
          lastName: '田中',
          email: 'taro.tanaka@example.com',
          phone: '090-1234-5678',
        },
        shippingAddress: {
          postalCode: '1234567', // Invalid postal code format
          prefecture: '東京都',
          city: '渋谷区',
          streetAddress: '渋谷1-1-1',
        },
        shippingMethodId: 'standard-shipping',
        paymentMethodId: 'credit-card',
      };

      return request(app.getHttpServer())
        .post('/orders')
        .send(invalidOrderRequest)
        .expect(400);
    });
  });

  describe('Error Handling', () => {
    it('should return structured error response', async () => {
      const response = await request(app.getHttpServer())
        .get('/nonexistent-endpoint')
        .expect(404);

      expect(response.body).toHaveProperty('statusCode');
      expect(response.body).toHaveProperty('message');
      expect(response.body).toHaveProperty('error');
      expect(response.body).toHaveProperty('timestamp');
      expect(response.body).toHaveProperty('path');
    });
  });
});
