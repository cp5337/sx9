db.createUser({
  user: 'ctas_user',
  pwd: process.env.MONGO_ROOT_PASSWORD,
  roles: [
    { role: 'readWrite', db: 'ctas' }
  ]
});