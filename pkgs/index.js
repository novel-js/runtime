const E = require('express')
const Fs = require('fs')
const { STATUS_CODES } = require('http')
const { stringify } = require('querystring')
const server = E()

server.get('/pkg/:bucket/:project', (req, res) => {
  const bucket_path = `/home/accusitive/Projects/personal/novel/pkgs/buckets/${req.params.bucket}`;
  const project_path =`/home/accusitive/Projects/personal/novel/pkgs/buckets/${req.params.bucket}/projects/${req.params.project}`;
  const project_dist =`/home/accusitive/Projects/personal/novel/pkgs/buckets/${req.params.bucket}/projects/${req.params.project}/dist.js`
    Fs.exists(bucket_path, (exists) => {
      if(!exists){
        return res.send("bucket does not exist")
      }
      Fs.exists(project_path, (exists) => {
        if(!exists){
          return res.send("Project does not exist")
        }
       
        res.sendFile(project_dist);

      })
    })
})
server.get('/pkg/:bucket/:project/file/:file', (req, res) => {
  const bucket_path = `/home/accusitive/Projects/personal/novel/pkgs/buckets/${req.params.bucket}`;
  const project_path =`/home/accusitive/Projects/personal/novel/pkgs/buckets/${req.params.bucket}/projects/${req.params.project}`;
  const file_path =`/home/accusitive/Projects/personal/novel/pkgs/buckets/${req.params.bucket}/projects/${req.params.project}/${req.params.file}`
    Fs.exists(bucket_path, (exists) => {
      if(!exists){
        return res.send("bucket does not exist")
      }
      Fs.exists(project_path, (exists) => {
        if(!exists){
          return res.send("Project does not exist")
        }
       
        res.sendFile(file_path);

      })
    })
})
server.listen(3030, () => {
  console.log('started')
})
