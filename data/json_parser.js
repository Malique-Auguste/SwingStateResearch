const fs = require('fs');

function extract_json(file_path) {
    console.log(file_path)

    fs.readFile(file_path, (err, data) => {
        if (!err) {
            const data_as_str = data.toString()
            var data_as_obj = JSON.parse(data_as_str)

            data_as_obj = data_as_obj['features'].map(constituency => {
                var name = ""

                if (file_path == "raw_communities.geojson") {
                    name = constituency['properties']['COMM_NAME']
                }
                else {
                    name = constituency['properties']['Constituency']
                }

                console.log(name)
                
                var coors = constituency['geometry']['coordinates'][0]

                coors = coors.map(coor => {                    
                    return [coor[1], coor[0]]
                })

                return {"name": name, "coors": coors}
            });

            const new_file_path = file_path.replace("raw", "processed")

            fs.writeFile(new_file_path, JSON.stringify(data_as_obj), (err) => {
                if (err) {
                  console.error(err);
                  return
                }

                console.log('Data written to file')
            })
        }
        else {
            console.log("ERR", err)
        }
    })
}

extract_json("raw_communities.geojson")
extract_json("raw_constituencies.json")
