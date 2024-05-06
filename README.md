# yamltoolrs

# YAMLToolRS
YAMLToolRS is a FAST and RELIABLE command-line tool for converting various application config files, such as appsettings.json for C#/ASP.NET Core and application.properties for SpringBoot/Java, to Kubernetes env format.

## Usage

```sh
yamltool <filePath> <language>

Arguments:
<filePath>  Path to the JSON or properties file this will be appsettings.json for .NET(C#) or application.properties for JAVA
<language>  Programming language option (--java or --csharp)

```

## Examples
```sh
yamltool examples/csharp/appsettings.json --csharp
```

```sh
yamltool examples/java/application.properties --java
```



