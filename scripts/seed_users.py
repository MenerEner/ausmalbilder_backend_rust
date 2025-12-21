import requests
import random
import string

def get_random_string(length):
    return ''.join(random.choices(string.ascii_lowercase, k=length))

def seed_users(count=1000):
    url = "http://localhost:8081/auth/signup"
    password = "Password123!"
    
    first_names = ["John", "Jane", "Alice", "Bob", "Charlie", "Diana", "Edward", "Fiona", "George", "Hannah"]
    last_names = ["Smith", "Doe", "Johnson", "Brown", "Williams", "Jones", "Garcia", "Miller", "Davis", "Rodriguez"]

    for i in range(count):
        first_name = random.choice(first_names)
        last_name = random.choice(last_names)
        random_suffix = get_random_string(5)
        email = f"{first_name.lower()}.{last_name.lower()}.{random_suffix}_{i}@example.com"
        
        payload = {
            "first_name": first_name,
            "last_name": last_name,
            "email": email,
            "password": password
        }
        
        try:
            response = requests.post(url, json=payload)
            if response.status_code == 201:
                if i % 100 == 0:
                    print(f"Created {i} users...")
            else:
                print(f"Failed to create user {i}: {response.status_code} - {response.text}")
        except Exception as e:
            print(f"Error creating user {i}: {e}")

if __name__ == "__main__":
    seed_users(1000)
