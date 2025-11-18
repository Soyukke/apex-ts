# apex-ts

A CLI tool for automatically generating TypeScript type definitions from Apex classes for Salesforce Lightning Web Components.

## Features

- 🚀 Export types with simple `@tsexport` annotation in JSDoc comments
- 🔒 Only outputs fields and methods with `@AuraEnabled` annotation
- ⚠️ Displays warnings for fields/methods without `@AuraEnabled`
- 📝 Automatically converts Apex types to TypeScript types
- 🔄 Supports generics like List, Set, Map
- ⚡ **Generates Salesforce LWC-compatible `@salesforce/apex` module declarations**
- 🎯 Type-safe Apex method calls from Lightning Web Components
- 📦 Recursive directory scanning
- 🔍 Debug logging with `tracing`

## Installation

### Pre-built binaries (Recommended)

Download the latest release for your platform from [GitHub Releases](https://github.com/YOUR_USERNAME/apex-ts/releases).

### Build from source

```bash
cargo build --release
```

The binary will be generated at `target/release/apex-ts`.

## Usage

### Basic Usage

```bash
apex-ts -i <input-directory> -o <output-file>
```

### Options

- `-i, --input <DIR>`: Directory containing Apex class files (.cls)
- `-o, --output <FILE>`: Output TypeScript file path (default: `types.d.ts`)
- `-v, --verbose`: Display detailed logs (shows warnings and debug information)
- `-h, --help`: Display help

### Quick Start

```bash
# Generate type definitions from your Apex classes
apex-ts -i force-app/main/default/classes -o lwc/types.d.ts

# With verbose output
apex-ts -i force-app/main/default/classes -o lwc/types.d.ts -v
```

## Examples

See the [examples directory](./examples) for complete examples of Apex classes with `@tsexport` annotations and the generated TypeScript definitions.

**Example structure:**
- [examples/](./examples) - Root level classes
- [examples/controllers/](./examples/controllers) - Controller classes
- [examples/services/](./examples/services) - Service layer classes
- [examples/models/](./examples/models) - Data models
- [output/types.d.ts](./output/types.d.ts) - Generated TypeScript definitions

### Basic Apex Class

```apex
/**
 * Account class
 * @tsexport
 */
public class Account {
    @AuraEnabled
    public String name;
    
    @AuraEnabled
    public Integer age;
    
    @AuraEnabled(cacheable=true)
    public static Account getAccountById(String accountId) {
        return null;
    }
}
```

### Generated TypeScript

```typescript
export interface Account {
  name: string;
  age: number;
}

declare module '@salesforce/apex/Account.getAccountById' {
  export default function getAccountById(params: {
    accountId: string;
  }): Promise<Account>;
}
```

### Usage in LWC

```javascript
import getAccountById from '@salesforce/apex/Account.getAccountById';

export default class MyComponent extends LightningElement {
  async loadAccount() {
    const account = await getAccountById({
      accountId: '001...'
    });
    console.log(account.name); // Type-safe!
  }
}
```

## Type Conversion Rules

| Apex Type | TypeScript Type |
|-----------|-----------------|
| String | string |
| Integer, Long, Double, Decimal | number |
| Boolean | boolean |
| Date, DateTime, Time | string |
| Id | string |
| List\<T\> | T[] |
| Set\<T\> | T[] |
| Map\<K, V\> | Record\<K, V\> |
| Object | any |
| Custom classes | As is |

## Annotation Rules

- **`@tsexport`**: Place in JSDoc comment (`/** @tsexport */`) above the class declaration
- **`@AuraEnabled`**: Standard Apex annotation, can be on separate line or same line
- **Supports**: `@AuraEnabled(cacheable=true)` and other parameters
- **One-line style**: `@AuraEnabled public String field;` is supported

## Development

### Running Tests

```bash
cargo test
```

### Testing with Examples

```bash
cargo run -- -i examples -o output/types.d.ts -v
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Releasing

To create a new release:

1. Update version in `Cargo.toml`
2. Commit the changes
3. Create and push a tag:
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```
4. GitHub Actions will automatically build binaries for all platforms and create a release

## License

MIT