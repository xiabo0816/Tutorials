<?php

namespace App\Http\Controllers;

use App\Http\Controllers\Controller;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Storage;
use Aws\S3\S3Client;  
use Aws\Exception\AwsException;
use Aws;

class UploadController extends Controller
{
    /*
     * UploadController需要先启动minio数据服务，操作详情见minio.md和minio.bat
     */

     // 这两个是minio的bucket名字, 需要先：1)创建，之后再 2)公开下载
    protected $name_bucket_img = 'images';
    protected $name_bucket_pdf = 'papers';

    // https://blog.csdn.net/a26637896/article/details/103156580
    public function uploadImg(Request $request)
    {
        $name_bucket_img = $this->name_bucket_img;

        $file = $request->file($name_bucket_img);
        $filename = $this->saveFile($file, $name_bucket_img);
        // wangEditor
        // $result["errno"] = 0;
        // $result["data"] = $savepath;
        // $result['url'] = "{$url}";
        // errno 即错误代码，0 表示没有错误。
        //       如果有错误，errno != 0，可通过下文中的监听函数 fail 拿到该错误码进行自定义处理
        // data 是一个数组，返回图片的线上地址
        $result = ["errno" => 0, "data" => [config('aws.endpoint').$filename,]];
        return $result;
    }

    public function uploadPdf(Request $request)
    {
        $name_bucket_pdf = $this->name_bucket_pdf;

        $file = $request->file($name_bucket_pdf);
        $filename = $this->saveFile($file, $name_bucket_pdf);

        // 此处的返回格式沿用了wangEditor的设计
        // errno 即错误代码，0 表示没有错误。
        //       如果有错误，errno != 0，可通过下文中的监听函数 fail 拿到该错误码进行自定义处理
        // data 是一个数组，返回图片的线上地址
        $result = ["errno" => 0, "data" => [config('aws.endpoint').$filename,]];
        return $result;
    }


    /**
     * 利用aws.s3上传到minio
     * 输入已经上传的文件路径和bucket名字
     * 输出上传后的文件路径
     * @return string
     */
    private function saveFile($file, $bucket = 'minioadmin')
    {
        $s3 = Aws::createClient('s3');
        $filename = time().'.'.$file->guessExtension();
        $s3_return = $s3->putObject([
            'Bucket' => $bucket, //存储桶
            'Key'    => $filename, //文件名
            'Body'   => file_get_contents($file) //要上传的文件
        ]);
        // if($s3_return['@metadata']['statusCode'] == 200){
        // } else {
        //     echo '返回值错误 : return fail! ';continue;
        // }

        // $disk = Storage::disk('qiniu'); //使用七牛云上传
        // $prefix = 'dongchedashen/userfile';
        // $filename = $disk->put($prefix, $img); //上传
        return '/'.$bucket.'/'.$filename;
    }

}
