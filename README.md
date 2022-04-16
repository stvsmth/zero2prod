# ZeroToProduction 

Based on the [Zero To Production book](https://www.zero2prod.com) ([repo](https://github.com/LukeMathWalker/zero-to-production))

## Spinning up on DigitalOcean (after creating an account)

```bash
export DO_ID=aaaaa

doctl apps create --spec app.yaml

# Migrate the database
# Turn of Trusted source
# ... <Find link, or better yet, `doctl` command or somehow automate>

# Migrate the database
# ... go to Settings and copy the connection string
DATABASE_URL="postgresql://newsletter:...?sslmode=require" sqlx migrate run

# Test DB
curl --request POST \
    --data 'name=le%20guin&email=ursula_le_guin%40gmail.com' \
    https://zero2prod-hkmtj.ondigitalocean.app/subscriptions \
    --verbose
```
